//! Progress spinner for long-running operations.

use crate::ui::constants::{
    color_dim, color_failure, color_success, ICON_CRITICAL, ICON_PASS, SPINNER_FRAMES,
};
use crate::ui::utils::is_tty;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

/// A spinner that runs on a background thread.
///
/// Shows animated progress while a long operation runs.
/// Automatically clears the spinner line when stopped.
pub struct Spinner {
    handle: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>,
}

impl Spinner {
    /// Start a new spinner with the given message.
    ///
    /// The spinner runs on a background thread and animates at 80ms per frame.
    /// On TTY, shows the animated spinner. On non-TTY, shows a static message.
    ///
    /// # Arguments
    /// * `message` - The message to display while spinning
    ///
    /// # Returns
    /// A `Spinner` instance
    pub fn start(message: &str) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);
        let message = message.to_string();

        let handle = if is_tty() {
            // TTY mode: animated spinner
            Some(thread::spawn(move || {
                let mut frame = 0;
                while !stop_clone.load(Ordering::Relaxed) {
                    let spinner_char = SPINNER_FRAMES[frame % SPINNER_FRAMES.len()];
                    eprint!("\r  {} {}", spinner_char, message);
                    frame += 1;
                }
                // Clear the spinner line
                eprint!("\r{}", " ".repeat(100));
                eprint!("\r");
            }))
        } else {
            // Non-TTY mode: single line
            eprintln!("{}", message);
            None
        };

        Spinner { handle, stop }
    }

    /// Stop the spinner and display a success message.
    ///
    /// Clears the spinner animation and prints a green check mark with the message.
    ///
    /// # Arguments
    /// * `mut self` - Consumes the spinner
    /// * `message` - The success message to display
    pub fn stop_with_success(mut self, message: &str) {
        self.stop.store(true, Ordering::Relaxed);

        // Wait for background thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        if is_tty() {
            eprintln!("  {}  {}", color_success(ICON_PASS), message);
        }
    }

    /// Stop the spinner and display a failure message.
    ///
    /// Clears the spinner animation and prints a red X with the message.
    ///
    /// # Arguments
    /// * `mut self` - Consumes the spinner
    /// * `message` - The failure message to display
    #[allow(dead_code)]
    pub fn stop_with_failure(mut self, message: &str) {
        self.stop.store(true, Ordering::Relaxed);

        // Wait for background thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        if is_tty() {
            eprintln!("  {}  {}", color_failure(ICON_CRITICAL), message);
        }
    }

    /// Stop the spinner and display a custom message.
    ///
    /// Clears the spinner animation and prints the message without icon.
    ///
    /// # Arguments
    /// * `mut self` - Consumes the spinner
    /// * `message` - The message to display
    #[allow(dead_code)]
    pub fn stop_with_message(mut self, message: &str) {
        self.stop.store(true, Ordering::Relaxed);

        // Wait for background thread to finish
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }

        if is_tty() {
            eprintln!("  {}", color_dim(message));
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinner_create() {
        let spinner = Spinner::start("Testing...");
        spinner.stop_with_message("Done");
        // Just verify it doesn't panic
    }

    #[test]
    fn test_spinner_success() {
        let spinner = Spinner::start("Processing...");
        spinner.stop_with_success("Completed successfully");
        // Just verify it doesn't panic
    }

    #[test]
    fn test_spinner_failure() {
        let spinner = Spinner::start("Analyzing...");
        spinner.stop_with_failure("Analysis failed");
        // Just verify it doesn't panic
    }
}
