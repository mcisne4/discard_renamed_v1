/// Structured `Error` response that can be parsed by the frontend
///
/// # Properties:
/// - `category`: `String` - The error response object category
/// - `message`: `String` - The main error response message to display
/// - `cause`: `String` - Further details of what caused the error
/// - `source`: `String` - The path to where the error occurred
///
/// # Methods:
/// - `new_error` - Creates a new *Error* response object
///
/// # Example:
/// ```
/// use rs_response::{DataResponse, ErrorResponse};
///
/// fn some_fn(condition: bool) -> DataResponse<u8> {
///   match condition {
///     true => Ok(15),
///     false => Err(
///       ErrorResponse::new_error(
///         "Test",
///         "This is an ERROR response",
///         "Caused for example purposes",
///         "rs_response::example::path()",
///       )
///     )
///   }
/// }
/// ```
#[derive(Debug, serde::Serialize)]
pub struct ErrorRepsonse {
    pub category: String,
    pub message: String,
    pub cause: String,
    pub source: String,
}
impl ErrorRepsonse {
    /// Creates a new *Error* response object for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The error response object category
    /// - `message`: `impl Into<String>` - The main error response message to display
    /// - `cause`: `impl Into<String>` - Further details of what caused the error
    /// - `source`: `impl Into<String>` - The path to where the error occurred
    ///
    /// # Example:
    /// ```
    /// use rs_response::{Response, ErrorResponse, OkResponse};
    ///
    /// fn some_fn(condition: bool) -> Response<u8> {
    ///   match condition {
    ///     true => Ok(
    ///       OkResponse::new_info(
    ///         "Test",
    ///         "This is an INFO response",
    ///         Some(12)
    ///       )
    ///     ),
    ///     false => Err(
    ///       ErrorResponse::new_error(
    ///         "Test",
    ///         "This is an ERROR response",
    ///         "Caused for example purposes",
    ///         "rs_response::example::path()",
    ///       )
    ///     )
    ///   }
    /// }
    /// ```
    pub fn new_error<Str: Into<String>>(
        category: Str,
        message: Str,
        cause: Str,
        source: Str,
    ) -> Self {
        Self {
            category: category.into(),
            message: message.into(),
            cause: cause.into(),
            source: source.into(),
        }
    }
}
