use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub(crate) enum EtymoraError {
    #[error("{0}")]
    ExampleAdapterError(#[source] example_adapter::ExampleError),
}
