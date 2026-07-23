use cirrus_error::error::CError;

#[derive(Clone)]
pub struct ToastError {
    /// Human readable error message.
    pub message: String,
    /// Full error details, including the actual error.
    pub error: String,
}

impl<E: CError> From<E> for ToastError {
    fn from(error: E) -> Self {
        Self {
            message: error.to_string(),
            error: format!("{:#?}", error)
        }
    }
}

#[derive(Clone)]
pub enum ToastText {
    String(String),
    Error(ToastError),
}

impl From<String> for ToastText {
    fn from(string: String) -> Self {
        Self::String(string)
    }
}

impl From<&str> for ToastText {
    fn from(string: &str) -> Self {
        Self::String(string.to_owned())
    }
}

impl From<Box<dyn CError>> for ToastText {
    fn from(error: Box<dyn CError>) -> Self {
        Self::Error(
            ToastError {
                message: error.to_string(),
                error: format!("{:?}", error)
            }
        )
    }
}