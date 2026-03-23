use std::fmt::Debug;

/// **NOW DEPRECATED:** Error trait from the cirrus tool kit 
/// to ease passing errors around the tool kit.
#[deprecated(note = "CError should no longer be used and may likely be removed soon. 
Instead of implementing `human_message()` implement `Display`, and instead of `actual_error()`
provide your actual error inside your `Error` enum as `error: String` such as `CosmicRayStruck { error: String }`.

A great example is the `core` library in Roseate: https://github.com/cloudy-org/roseate/blob/main/core/src/error.rs

And also the Roseate `app` itself, minus the CError trait impl: https://github.com/cloudy-org/roseate/blob/main/app/src/error.rs")]
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
    #[deprecated(note = "This is deprecated, use `Display` instead!")]
    fn human_message(&self) -> String;
    /// This should be a stringified version of the actual error that occurred.
    /// E.g: `error.detail()`, `error.msg()`
    #[deprecated(note = "This is deprecated, provide an `error` string type in the enum instead!")]
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