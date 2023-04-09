use super::Data;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub enum ErrorType<T> {
    Other(Option<T>),
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Error<T> {
    error_type: ErrorType<T>,
}

impl<T> Error<T> {
    pub fn new(error_type: ErrorType<T>) -> Self {
        Self { error_type }
    }
}

impl<T> Data for Error<T> where T: Data {}
