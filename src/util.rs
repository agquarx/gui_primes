//! Clipboard helper (only compiles with `--features clipboard`).

#[cfg(feature = "clipboard")]
use arboard::Clipboard;

use crate::primes::PrimeError;

#[cfg(feature = "clipboard")]
pub fn copy_to_clipboard(text: &str) -> Result<(), PrimeError> {
    Clipboard::new()
        .map_err(|_| PrimeError::Fatal("clipboard init"))?
        .set_text(text.to_owned())
        .map_err(|_| PrimeError::Fatal("clipboard set"))
}

#[cfg(not(feature = "clipboard"))]
pub fn copy_to_clipboard(_: &str) -> Result<(), PrimeError> {
    Err(PrimeError::Fatal("clipboard feature disabled"))
}