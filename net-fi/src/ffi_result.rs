use std::panic::UnwindSafe;

/// A trait to implement for result types of foreign functions.
/// Typically, this would be implemented for an enum or integer.
pub trait FFIResult {
    /// Executes the provided function and catches any panics, returning an
    /// appropriate NetFiResult value.
    fn catch(f: impl FnOnce() -> Self + UnwindSafe) -> Self;

    /// Returns a value that signifies a null-argument error.
    fn null_arg_error() -> Self;
}