mod async_iter;
mod benchmark;
mod error;
mod forms;
mod keyboard;
mod mouse;
mod multiplex;
mod process;
mod signal;
mod stdio;
mod table;
mod terminal;

pub use async_iter::*;
pub use benchmark::*;
pub use error::*;
pub use forms::*;
pub use keyboard::*;
pub use mouse::*;
pub use multiplex::*;
pub use process::*;
pub use signal::*;
pub use stdio::*;
pub use table::*;
pub use terminal::*;

use napi_derive::napi;

/// Gets the version of the stdio-napi package.
#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
