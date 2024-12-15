use lsp_server::{ErrorCode, ResponseError};
use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, EtymoraError>;

#[derive(Debug, Error)]
pub(crate) enum EtymoraError {
    #[error("{0}")]
    ExampleAdapterError(#[source] example_adapter::ExampleError),
    #[error("{0}")]
    ProcotolError(#[source] lsp_server::ProtocolError),
    #[error("Error occurs in desirializing, this is a type of ProtocolError: {0}")]
    DesirializeError(#[source] serde_json::Error),
    #[error("Error occurs in stdio: {0}")]
    StdIOError(#[source] std::io::Error),
    #[error("Sending a message failed. The message: {0:?}")]
    SendMassageError(lsp_server::Message),
}

impl From<&EtymoraError> for ErrorCode {
    fn from(value: &EtymoraError) -> Self {
        match value {
            EtymoraError::ExampleAdapterError(_) => ErrorCode::InternalError,
            EtymoraError::ProcotolError(_) => ErrorCode::InvalidRequest,
            EtymoraError::DesirializeError(_) => ErrorCode::InvalidParams,
            EtymoraError::StdIOError(_) => ErrorCode::InternalError,
            EtymoraError::SendMassageError(_) => ErrorCode::InternalError,
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
