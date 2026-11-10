//! A simple keycode parser for handling UTF-8 and VT100 terminal control sequences.
//!
//! # Example
//! 
//! Create a new parser instance using `serkey::Parser::new()`.
//!
//! ```rust
//! let mut serkey_parser = serkey::Parser::new();
//! ```
//!
//! `serkey::Reader::read_key` reads a key asynchronously from the reader supplied to `Reader::new`.
//!
//! ```rust
//! use serkey::ReadKey as _;
//!
//! async fn print_key_async(reader: impl embedded_io_async::Read) {
//!     let mut serkey_reader = serkey::Reader::new(reader);
//!     loop {
//!         if let Ok(key) = serkey_reader.read_key().await {
//!             match key {
//!                 serkey::Key::CookedChar('a') => println!("'a' is pressed"),
//!                 serkey::Key::CookedCtrl(0x01) => println!("Ctrl-A is pressed"),
//!                 serkey::Key::CookedCtrl(0x03) => break,
//!                 serkey::Key::Left(attr) => println!("[Left] is pressed with {:?}", attr),
//!                 serkey::Key::Delete(attr) => println!("[Delete] is pressed with {:?}", attr),
//!                 _ => println!("{}", key),
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! `serkey::Parser::read_key_blocking` reads a key from the given reader synchronously.
//!
//! ```rust
//! fn print_key_blocking(mut reader: impl embedded_io::Read) {
//!     let mut serkey_reader = serkey::ReaderBlocking::new(reader);
//!     loop {
//!         if let Ok(key) = serkey_reader.read_key_blocking() {
//!             match key {
//!                 serkey::Key::CookedChar('a') => println!("'a' is pressed"),
//!                 serkey::Key::CookedCtrl(0x01) => println!("Ctrl-A is pressed"),
//!                 serkey::Key::CookedCtrl(0x03) => break,
//!                 serkey::Key::Left(attr) => println!("[Left] is pressed with {:?}", attr),
//!                 serkey::Key::Delete(attr) => println!("[Delete] is pressed with {:?}", attr),
//!                 _ => println!("{}", key),
//!             }
//!         }
//!     }
//! }
//! ```
#![no_std]

mod serkey;
pub use serkey::*;
