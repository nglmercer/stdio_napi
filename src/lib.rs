mod stdio;
mod terminal;
mod process;
mod error;

pub use stdio::*;
pub use terminal::*;
pub use process::*;
pub use error::*;

use napi_derive::napi;

/// Gets the version of the stdio-napi package.
///
/// # Returns
/// * `String` - The semantic version string (e.g., "1.0.0")
///
/// # Example
/// ```javascript
/// const { get_version } = require('stdio-napi');
/// console.log(get_version()); // "1.0.0"
/// ```
#[napi]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
