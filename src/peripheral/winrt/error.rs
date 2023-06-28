use crate::{Error, ErrorType};
use futures::channel::mpsc::SendError;
use std::any::Any;
use windows::core::{HRESULT, HSTRING};

impl From<windows::core::Error> for Error {
    fn from(value: windows::core::Error) -> Self {
        Error::new(
            format!("windows::core::Error: {:?}", value.code()),
            format!("{:?}", value),
            ErrorType::Windows,
        )
    }
}

impl From<SendError> for Error {
    fn from(value: SendError) -> Self {
        Error::new(
            "futures::channel::mpsc::SendError",
            format!("{:?}", value),
            ErrorType::Windows,
        )
    }
}

impl From<Error> for windows::core::Error {
    fn from(value: Error) -> Self {
        windows::core::Error::new(HRESULT(2), HSTRING::from(format!("{:?}", value)))
    }
}
