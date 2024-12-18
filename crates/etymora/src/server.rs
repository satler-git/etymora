//! Server

use either::Either;

use lsp_server::{
    Connection, ExtractError, IoThreads, Message, RequestId, Response, ResponseError,
};
use lsp_types::{
    request::{HoverRequest, Request as _},
    Hover, HoverParams, HoverProviderCapability, InitializeParams, ServerCapabilities,
};

use crate::{
    dict_handler,
    error::{EtymoraError, Result},
    text_document::{self, FileSystem},
};

use tracing::{debug, info};

/// Server State
pub(crate) struct Etymora {
    connection: Connection,
    io_threads: IoThreads,
    params: InitializeParams, // TODO: user config
    dict: Option<dict_handler::Dicts>,
    fs: FileSystem,
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
                    work_done_progress: Some(true),
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
            Ok(it) => serde_json::from_value(it).map_err(EtymoraError::DesirializeError)?,
            Err(e) => {
                if e.channel_is_disconnected() {
                    io_threads.join().map_err(EtymoraError::StdIOError)?;
                }
                return Err(EtymoraError::ProcotolError(e));
            }
        };

        Ok(Etymora {
            connection,
            io_threads,
            params,
            fs: FileSystem::default(),
            dict: None,
        })
    }

    pub(crate) fn shutdown(self) -> Result<()> {
        info!("Shutting down server");

        self.io_threads.join().map_err(EtymoraError::StdIOError)
    }

    pub(crate) async fn main_loop(&self) -> Result<()> {
        for msg in &self.connection.receiver {
            // handle shutdown
            match &msg {
                Message::Request(req) => {
                    if self
                        .connection
                        .handle_shutdown(&req)
                        .map_err(EtymoraError::ProcotolError)?
                    {
                        return Ok(());
                    }
                }
                _ => (),
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

    pub(crate) async fn handle_hover(&self, _params: HoverParams) -> Result<Hover> {
        todo!() // TODO:
    }

    fn dispacth<R>(&self, res: Either<ResponseError, R>, id: RequestId) -> Result<()>
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
                let result = serde_json::to_value(&Some(result)).unwrap();
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
            .map_err(|e| EtymoraError::SendMassageError(e.0))?;

        Ok(())
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
