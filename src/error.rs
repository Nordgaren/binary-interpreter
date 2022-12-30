use std::string::{FromUtf16Error, FromUtf8Error};
use thiserror::Error as ThisError;


#[derive(ThisError, Debug)]
pub enum InterpreterError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Utf8Error(#[from] FromUtf8Error),
    #[error(transparent)]
    Utf16Error(#[from] FromUtf16Error)
}

