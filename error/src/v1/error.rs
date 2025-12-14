use std::fmt::Debug;

/// Error trait from the cirrus tool kit 
/// to ease passing errors around the tool kit.
pub trait CError: Debug {
    /// This should return a human readable message about the error that just occurred.
    /// It MUST also be easily interpretable by a non technical user as this is what 
    /// will be displayed to the user on the GUI.
    /// 
    /// Also if you're including the actual error in the message please format 
    /// it like so where the actual error is pushed to a new line:
    /// ```ignore
    /// fn human_message(&self) -> String {
    ///     match self {
    ///         Error::FailedToSaveConfig(actual_error) => format!(
    ///             "Failed to save config toml file! \n\nError: {}", actual_error
    ///         ),
    ///     }
    /// }
    /// ```
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