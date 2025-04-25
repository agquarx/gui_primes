//! Error types used by the prime calculation system

/// Errors that can occur during prime number calculations
#[derive(Debug)]
pub enum PrimeError {
    /// Operation would cause numeric overflow
    Overflow(&'static str),
    /// Fatal error that cannot be recovered from
    Fatal(&'static str),
    /// Error during execution of a calculation
    ExecutionError(String),
    /// Error related to clipboard operations
    ClipboardError(String),
}
