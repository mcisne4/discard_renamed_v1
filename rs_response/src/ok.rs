#[derive(Debug, serde::Serialize)]
pub struct Info<T> {
    pub category: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, serde::Serialize)]
pub struct Warning<T> {
    pub category: String,
    pub message: String,
    pub data: Option<T>,
    pub cause: String,
    pub source: String,
}

/// Structured `Ok` response that can be parsed by the frontend.
/// The data is formatted as either an *Informational* message
/// or a *Warning* message
///
/// # INFO Properties:
/// - `category`: `String` - The response object category
/// - `message`: `String` - The main response message to displayed
/// - `data`: `Option<T>` - Optional data that can be returned to the frontend
///
/// # WARN Properties:
/// - `category`: `String` - The response object category
/// - `message`: `String` - The main response message to displayed
/// - `data`: `Option<T>` - Optional data that can be returned to the frontend
/// - `cause`: `String` - Further details of what caused the warning
/// - `source`: `String` - The path to where the warning occurred
///
/// # Methods:
/// - `new_info` - Creates a new *Informational* response object
/// - `new_warning` - Creates a new *Warning* response object
///
/// # Example:
/// ```
/// use rs_response::{ResponseVec, OkResponse};
///
/// fn some_fn() -> ResponseVec<usize> {
///   let info = OkResponse::new_info(
///     "Test",
///     "This is an INFO result",
///     Some(34_usize),
///   );
///
///   let warn = OkResponse::new_warning(
///     "Test",
///     "This is a WARNING result",
///     None,
///     "Caused for example purposes",
///     "rs_response::example::path()"
///   );
///
///   Ok(
///     vec![info, warn]
///   )
/// }
/// ```
#[derive(Debug, serde::Serialize)]
pub enum OkResponse<T> {
    INFO(Info<T>),
    WARN(Warning<T>),
}
impl<T> OkResponse<T> {
    /// Creates a new *Informational* response object for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    /// - `data`: `Option<T>` - Optional data that can be returned to the frontend
    ///
    /// # Example:
    /// ```
    /// use rs_response::{Response, OkResponse};
    ///
    /// fn some_fn() -> Response<bool> {
    ///   Ok(
    ///     OkResponse::new_info(
    ///       "Test",
    ///       "This is an INFO result",
    ///       Some(true),
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_info<Str: Into<String>>(category: Str, message: Str, data: Option<T>) -> Self {
        Self::INFO(Info {
            category: category.into(),
            message: message.into(),
            data: data,
        })
    }

    /// Creates a new *Warning* response object for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    /// - `data`: `Option<T>` - Optional data that can be returned to the frontend
    /// - `cause`: `impl Into<String>` - Further details of what caused the warning
    /// - `source`: `impl Into<String>` - The path to where the warning occurred
    ///
    /// # Example:
    /// ```
    /// fn some_fn() -> Response<String> {
    ///   Ok(
    ///     OkResponse::new_warning(
    ///       "Test",
    ///       "This is a WARNING result",
    ///       Some(String::from("Hello World")),
    ///       "Caused for example purposes",
    ///       "rs_response::example::path()"
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_warning<Str: Into<String>>(
        category: Str,
        message: Str,
        data: Option<T>,
        cause: Str,
        source: Str,
    ) -> Self {
        Self::WARN(Warning {
            category: category.into(),
            message: message.into(),
            data: data,
            cause: cause.into(),
            source: source.into(),
        })
    }
}
