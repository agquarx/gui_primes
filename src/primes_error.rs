#[derive(Debug)]
pub enum PrimeError {
    Overflow(&'static str),
    Fatal(&'static str),
    ExecutionError(String),
    ClipboardError(String),
}
