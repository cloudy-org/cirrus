use std::fmt::{Debug, Display};

/// Error trait from the cirrus tool kit you should 
/// implement to ease passing errors around the tool kit.
pub trait CError: Display + Debug {}

impl<E> From<E> for Box<dyn CError>
where
    E: CError + 'static,
{
    fn from(error: E) -> Self {
        Box::new(error)
    }
}