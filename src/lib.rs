mod stdio;
mod terminal;
mod process;
mod error;

pub use stdio::*;
pub use terminal::*;
pub use process::*;
pub use error::*;

use napi_derive::napi;

#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
