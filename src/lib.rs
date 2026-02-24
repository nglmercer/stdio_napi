mod stdio;
mod terminal;
mod process;

pub use stdio::*;
pub use terminal::*;
pub use process::*;

use napi_derive::napi;

#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
