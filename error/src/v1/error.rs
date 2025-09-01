/// Error trait from the cirrus tool kit 
/// to ease passing errors around the tool kit.
pub trait CError {
    /// This should return a human readable message about the error that just occurred.
    /// It MUST also be easily interpretable by a non technical user as this is what 
    /// will be displayed to the user on the GUI.
    fn human_message(&self) -> String;
    /// This should be a stringified version of the actual error that occurred.
    /// E.g: `error.detail()`, `error.msg()`
    fn actual_error(&self) -> Option<String>;
}

impl<E> From<E> for Box<dyn CError>
where
    E: CError + 'static,
{
    fn from(error: E) -> Self {
        Box::new(error)
    }
}