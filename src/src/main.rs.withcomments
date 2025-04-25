<search/>
mod gui;
mod prime;

use crate::{PrimeType, PrimeError};

use std::sync::{Arc, Mutex, atomic::AtomicBool};

#[cfg(feature = "clipboard")]
pub(crate) fn copy_to_clipboard(text: &str) -> Result<(), PrimeError> {
    use arboard::Clipboard;
    let mut clipboard = Clipboard::new()
        .map_err(|e| PrimeError::ClipboardError(e.to_string()))?;

    clipboard.set_text(text)
        .map_err(|e| PrimeError::ClipboardError(e.to_string()))
}

pub use gui::run_gui;

fn main() -> eframe::Result<()> {
    gui::run_gui()
}

