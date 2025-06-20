//! Contains traits that define common behaviors across the system.

/// The `Executable` trait defines types that can be executed to produce a result.
///
/// This trait is designed to be implemented by any type that can perform
/// some kind of operation when requested and return a result of any type.
pub trait Executable<R> {
    /// Execute the implementing type and produce a result of type `R`.
    ///
    /// # Returns
    ///
    /// A value of type `R` that represents the result of execution.
    fn execute(&self) -> R;
}

pub trait NodeContent {
    /// The type of output produced when this content is executed
    type Output;

    /// Returns a unique identifier for this content
    fn id(&self) -> String;

    /// Returns a list of dependencies by their identifiers
    fn dependencies(&self) -> Vec<String>;

    /// Execute this content and produce its output
    fn execute(&self) -> Self::Output;
}
