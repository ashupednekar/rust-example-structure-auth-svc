use thiserror::Error;

#[derive(Debug, Error)]
pub enum CustomError{
    #[error("some variant")]
    SomeVariant
}

pub type Result<T> = core::result::Result<T, CustomError>;

