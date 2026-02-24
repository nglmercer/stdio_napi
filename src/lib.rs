mod error;
mod forms;
mod keyboard;
mod process;
mod stdio;
mod table;
mod terminal;

pub use error::*;
pub use forms::*;
pub use keyboard::*;
pub use process::*;
pub use stdio::*;
pub use table::*;
pub use terminal::*;

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
