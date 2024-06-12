#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

//! Crossterm KeyReader is a library crate for handling key input events using the crossterm crate in Rust.
//!
//! This crate provides an interface to easily capture and process key inputs in terminal applications.
//!
//! ## Features
//!
//! - Simple API: KeyReader offers a simple and intuitive API for processing key input events.
//! - Asynchronous Support: Supports asynchronous programming for efficient input handling.
//! - Cross-Platform: Built on top of the Crossterm crate, it works on Windows, Linux, and macOS.
//!
//! ## Installation
//!
//! Add the following dependency to your Cargo.toml:
//! ```text
//! [dependencies]
//! crossterm-keyreader = "0.1"
//! ```
//!
//! ## Example
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() {
//!     let mut rc = crossterm_keyreader::spawn();
//!     loop {
//!         if let Ok(event) = rc.try_recv() {
//!             println!("KeyEvent is: {:?}", event);
//!         }
//!     }
//! }
//! ```

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEvent;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;

/// Spawns a channel to handle key input events asynchronously.
///
/// This function spawns a new Tokio task that asynchronously reads key input events
/// from the terminal. The events are sent through a channel, and you can receive
/// these events using the returned receiver.
///
/// # Returns
///
/// `Receiver<KeyEvent>` - An asynchronous receiver for key events.
///
/// # Example
///
/// ```ignore
/// #[tokio::main]
/// async fn main() {
///     let mut rc = crossterm_keyreader::spawn();
///     loop {
///         if let Ok(event) = rc.try_recv() {
///             println!("KeyEvent is: {:?}", event);
///         }
///     }
/// }
/// ```
///
/// # Note
///
/// - This function must be used within a Tokio runtime.
/// - The channel buffer size is set to 100. If this capacity is exceeded, an error will occur.
pub fn spawn() -> Receiver<KeyEvent> {
    let (tx, rx) = mpsc::channel::<KeyEvent>(100);

    tokio::spawn(async move {
        loop {
            if let Ok(event) = event::read() {
                if let Event::Key(event) = event {
                    tx.send(event)
                        .await
                        .expect("keyreader buffer capacity reached.");
                }
            }
        }
    });

    rx
}
