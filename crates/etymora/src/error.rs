use lsp_server::{ErrorCode, ResponseError};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, EtymoraError>;

#[derive(Debug, Error)]
pub(crate) enum EtymoraError {
    #[error("{0}")]
    ExampleAdapter(#[source] adapter_example::ExampleError),
    #[error("{0}")]
    Protocol(#[source] lsp_server::ProtocolError),
    #[error("Error occurs in desirializing, this is a type of ProtocolError: {0}")]
    Desirialize(#[source] serde_json::Error),
    #[error("Error occurs in stdio: {0}")]
    StdIO(#[source] std::io::Error),
    #[error("Sending a message failed. The message: {0:?}")]
    SendMessage(lsp_server::Message),
    #[error("{0}")]
    Fs(crate::text_document::FsError),
}

impl From<&EtymoraError> for ErrorCode {
    fn from(value: &EtymoraError) -> Self {
        match value {
            EtymoraError::ExampleAdapter(_) => ErrorCode::InternalError,
            EtymoraError::StdIO(_) => ErrorCode::InternalError,
            EtymoraError::SendMessage(_) => ErrorCode::InternalError,

            EtymoraError::Protocol(_) => ErrorCode::InvalidRequest,

            EtymoraError::Desirialize(_) => ErrorCode::InvalidParams,
            EtymoraError::Fs(_) => ErrorCode::InvalidParams,
        }
    }
}

impl From<EtymoraError> for ResponseError {
    fn from(value: EtymoraError) -> Self {
        Self {
            code: Into::<lsp_server::ErrorCode>::into(&value) as i32,
            message: format!("{}", value),
            data: None,
        }
    }
}
