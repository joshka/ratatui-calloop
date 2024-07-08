//! Ratatui-calloop is an experimental library for building terminal applications using the [Calloop]
//! event loop.
//!
//! [Calloop]: https://docs.rs/calloop
//!
//! This library provides a simple API for building terminal applications using the calloop event
//! loop. It is built on top of the ratatui library, which provides a high-level API for building
//! terminal applications. The ratatui-calloop library provides an implementation of the `App` trait
//! that is compatible with the calloop event loop, allowing you to build terminal applications that
//! respond to user input and update the terminal in real-time.
//!
//! # Example
//!
//! The following is a simple example of a terminal application that displays a counter and responds
//! to key events to increment or decrement the counter. The application uses the ratatui-calloop
//! library to handle the event loop and update the terminal.
//!
//! ```no_run
#![doc = include_str!("../examples/demo.rs")]
//! ```
//!
//! # Features
#![doc = document_features::document_features!()]

pub use app::{App, ApplicationLoop};
pub use error::{Error, Result};
pub use terminal::Terminal;

mod app;
mod crossterm;
mod error;
mod terminal;
