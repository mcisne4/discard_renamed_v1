use serde::Serialize;

/// # Properties:
/// - `error`: `String` - Main description of the error
/// - `info`: `String` - More information about the error
#[derive(Debug, Serialize)]
pub struct RenamedError {
    pub error: String,
    pub info: String,
}

/// Type alias: `RenamedResult`
///
/// # Example:
/// ```
/// use rs_errors::{RenamedResult, create_error};
///
/// fn sample_fn() -> RenamedResult<()> {
///   match some_result {
///     Ok(_) => Ok(()),
///     Err(e) => Err(create_error("Error", e)),
///   }
/// }
/// ```
pub type RenamedResult<T> = std::result::Result<T, RenamedError>;

/// Creates a `RenamedError`
/// # Arguments:
/// - `error` - Main description of the error
/// - `info` - More information about the error
///
/// # Example:
/// ```
/// use rs_errors::{RenamedResult, create_error};
///
/// fn sample_fn() -> RenamedResult<()> {
///   match some_result {
///     Ok(_) => Ok(()),
///     Err(e) => Err(create_error("Error", e)),
///   }
/// }
/// ```
pub fn create_error(error: impl Into<String>, info: impl Into<String>) -> RenamedError {
    RenamedError {
        error: error.into(),
        info: info.into(),
    }
}
