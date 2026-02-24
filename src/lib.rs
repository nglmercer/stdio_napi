mod async_iter;
mod benchmark;
mod error;
mod forms;
mod keyboard;
mod macos_api;
mod mouse;
mod multiplex;
mod process;
mod pty;
mod signal;
mod stdio;
mod table;
mod terminal;
mod windows_api;

pub use async_iter::*;
pub use benchmark::*;
pub use error::*;
pub use forms::*;
pub use keyboard::*;
pub use macos_api::*;
pub use mouse::*;
pub use multiplex::*;
pub use process::*;
pub use pty::*;
pub use signal::*;
pub use stdio::*;
pub use table::*;
pub use terminal::*;
pub use windows_api::*;

use napi_derive::napi;

/// Gets the version of the stdio-napi package.
#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
