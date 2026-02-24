mod stdio;
mod terminal;

pub use stdio::*;
pub use terminal::*;

use napi_derive::napi;

#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
