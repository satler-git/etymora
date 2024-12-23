//! Server

use either::Either;

use etymora_traits::Dictionary;
use lsp_server::{
    Connection, ExtractError, IoThreads, Message, Notification, RequestId, Response, ResponseError,
};
use lsp_types::{
    notification::Notification as _,
    request::{HoverRequest, Request as _},
    Hover, HoverParams, HoverProviderCapability, InitializeParams, MarkupContent, NumberOrString,
    ServerCapabilities, WorkDoneProgressReport,
};

use crate::{
    dict_handler,
    error::{EtymoraError, Result},
    text_document::FileSystem,
};

use tracing::{debug, info, warn};

/// Server State
pub(crate) struct Etymora {
    connection: Connection,
    io_threads: IoThreads,
    #[allow(dead_code)]
    config: Config,
    dict: Option<dict_handler::Dicts>,
    fs: FileSystem,
}

#[derive(Debug, serde::Deserialize, Default)]
struct Config {
    dict_config: dict_handler::DictConfigs,
}

impl Etymora {
    /// Generate(static) Server Capabilities
    /// `..Default::default()` cannot be used in a const context.
    /// * `HoverProvider` with `WorkDoneProgress`
    #[inline]
    fn gen_server_capabilities() -> ServerCapabilities {
        ServerCapabilities {
            hover_provider: Some(HoverProviderCapability::Options(lsp_types::HoverOptions {
                work_done_progress_options: lsp_types::WorkDoneProgressOptions {
                    work_done_progress: Some(false),
                },
            })),
            ..Default::default()
        }
    }

    pub(crate) async fn init() -> Result<Etymora> {
        info!("Starting LSP server");

        let server_capabilities = serde_json::to_value(Self::gen_server_capabilities()).unwrap();

        let (connection, io_threads) = Connection::stdio();

        let params: InitializeParams = match connection.initialize(server_capabilities) {
            Ok(it) => serde_json::from_value(it).map_err(EtymoraError::Desirialize)?,
            Err(e) => {
                if e.channel_is_disconnected() {
                    io_threads.join().map_err(EtymoraError::StdIO)?;
                }
                return Err(EtymoraError::Protocol(e));
            }
        };

        let config: Config = if let Some(options) = params.initialization_options {
            serde_json::from_value(options).map_err(EtymoraError::Desirialize)?
        } else {
            warn!("InitializeOptions is empty. Using the default Config");
            Config::default()
        };

        let dict = Some(dict_handler::Dicts::init(&config.dict_config).await?);

        Ok(Etymora {
            connection,
            io_threads,
            config,
            dict,
            fs: FileSystem::default(),
        })
    }

    pub(crate) fn shutdown(self) -> Result<()> {
        info!("Shutting down server");

        self.io_threads.join().map_err(EtymoraError::StdIO)
    }

    pub(crate) async fn main_loop(&self) -> Result<()> {
        for msg in &self.connection.receiver {
            // handle shutdown
            if let Message::Request(req) = &msg {
                if self
                    .connection
                    .handle_shutdown(req)
                    .map_err(EtymoraError::Protocol)?
                {
                    return Ok(());
                }
            }

            debug!("Got message: {msg:?}");

            self.massage_handler(msg).await?;
        }

        Ok(())
    }

    pub(crate) async fn massage_handler(&self, msg: Message) -> Result<()> {
        match msg {
            Message::Request(req) => match req.method.as_str() {
                HoverRequest::METHOD => match cast::<HoverRequest>(req) {
                    Ok((id, params)) => {
                        let res = self.handle_hover(params).await;

                        let res = if res.is_err() {
                            Either::Left(res.err().unwrap().into())
                        } else {
                            Either::Right(res.ok().unwrap())
                        };

                        self.dispacth(res, id)?;

                        Ok(())
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => panic!("{req:?}"),
                },
                _ => Ok(()),
            },
            Message::Response(_) => Ok(()),
            Message::Notification(_) => Ok(()),
        }
    }

    pub(crate) async fn handle_hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        info!("Handling hover");

        let word = self
            .fs
            .read_word_uri(
                &params.text_document_position_params.text_document.uri,
                &params.text_document_position_params.position,
            )
            .await
            .map_err(EtymoraError::Fs)?;

        if word.is_none() | self.dict.is_none() {
            // ワードがない場合はなにもなく返す
            info!("No word or dictionary found");
            return Ok(None);
        }

        let word = word.unwrap();

        let desc = self.dict.as_ref().unwrap().lookup_ditail(&word).await?;

        if desc.is_none() {
            // 説明がない場合はなにもなく返す
            info!("No description found");
            return Ok(None);
        }

        let desc = lsp_types::HoverContents::Markup(MarkupContent {
            kind: lsp_types::MarkupKind::Markdown,
            value: desc.unwrap(),
        });

        let resp = Hover {
            contents: desc,
            range: None, // TODO: `read_word_uri` の構造を変えないと対応できない
        };

        Ok(Some(resp))
    }

    fn dispacth<R>(&self, res: Either<ResponseError, Option<R>>, id: RequestId) -> Result<()>
    where
        R: serde::Serialize,
    {
        let resp = match res {
            Either::Left(error) => Response {
                id,
                result: None,
                error: Some(error),
            },
            Either::Right(result) => {
                let result = serde_json::to_value(&result).unwrap();
                Response {
                    id,
                    result: Some(result),
                    error: None,
                }
            }
        };

        self.connection
            .sender
            .send(Message::Response(resp))
            .map_err(|e| EtymoraError::SendMessage(e.0))
    }

    #[allow(dead_code)]
    fn progress(&self, token: NumberOrString, message: String) -> Result<()> {
        let noti = Notification {
            method: lsp_types::notification::Progress::METHOD.into(),
            params: serde_json::to_value(lsp_types::ProgressParams {
                token,
                value: lsp_types::ProgressParamsValue::WorkDone(
                    lsp_types::WorkDoneProgress::Report(WorkDoneProgressReport {
                        message: Some(message),
                        ..Default::default()
                    }),
                ),
            })
            .unwrap(),
        };

        self.connection
            .sender
            .send(Message::Notification(noti))
            .map_err(|e| EtymoraError::SendMessage(e.0))
    }

    #[allow(dead_code)]
    fn progress_start(&self, token: NumberOrString, title: String) -> Result<()> {
        self.connection
            .sender
            .send(lsp_server::Message::Notification(
                lsp_server::Notification {
                    method: lsp_types::notification::Progress::METHOD.into(),
                    params: serde_json::to_value(lsp_types::ProgressParams {
                        token,
                        value: lsp_types::ProgressParamsValue::WorkDone(
                            lsp_types::WorkDoneProgress::Begin(lsp_types::WorkDoneProgressBegin {
                                title,
                                ..Default::default()
                            }),
                        ),
                    })
                    .unwrap(),
                },
            ))
            .map_err(|e| EtymoraError::SendMessage(e.0))
    }

    #[allow(dead_code)]
    fn progress_end(&self, token: NumberOrString, message: Option<String>) -> Result<()> {
        self.connection
            .sender
            .send(lsp_server::Message::Notification(
                lsp_server::Notification {
                    method: lsp_types::notification::Progress::METHOD.into(),
                    params: serde_json::to_value(lsp_types::ProgressParams {
                        token,
                        value: lsp_types::ProgressParamsValue::WorkDone(
                            lsp_types::WorkDoneProgress::End(lsp_types::WorkDoneProgressEnd {
                                message,
                            }),
                        ),
                    })
                    .unwrap(),
                },
            ))
            .map_err(|e| EtymoraError::SendMessage(e.0))
    }
}

fn cast<R>(
    req: lsp_server::Request,
) -> std::result::Result<(lsp_server::RequestId, R::Params), ExtractError<lsp_server::Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
