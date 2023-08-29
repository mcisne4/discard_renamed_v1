use crate::ok_without_data::{Info, OkResponse, Warning};

#[derive(Debug, serde::Serialize)]
pub struct InfoWithData<T> {
    pub category: String,
    pub message: String,
    pub data: T,
}

#[derive(Debug, serde::Serialize)]
pub struct WarningWithData<T> {
    pub category: String,
    pub message: String,
    pub data: T,
    pub cause: String,
    pub source: String,
}

/// Structured `Ok` response with additional data that can be parsed by the frontend.
/// The data is formatted as either an *Informational* message
/// or a *Warning* message
///
/// # INFO Properties:
/// - `category`: `String` - The response object category
/// - `message`: `String` - The main response message to displayed
/// - `data`: `T` - Additional data to return to the frontend
///
/// # WARN Properties:
/// - `category`: `String` - The response object category
/// - `message`: `String` - The main response message to displayed
/// - `data`: `T` - Additional data to return to the frontend
/// - `cause`: `String` - Further details of what caused the warning
/// - `source`: `String` - The path to where the warning occurred
///
/// # Methods:
/// - `new_info` - Creates a new *Informational* response object without additional *data*
/// - `new_warning` - Creates a new *Warning* response object without additional *data*
/// - `drop_data` - Drops the additional data, converting the `OkDataResponse` to an `OkResponse`
///
/// # Example:
/// ```
/// use rs_response::{ResponseVecWithData, OkResponse};
///
/// fn some_fn() -> ResponseVecWithData<usize> {
///   let info = OkResponse::new_info_with_data(
///     "Test",
///     "This is an INFO result",
///     34_usize,
///   );
///
///   let warn = OkResponse::new_warning_with_data(
///     "Test",
///     "This is a WARNING result",
///     16_usize,
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
pub enum OkDataResponse<T> {
    INFOData(InfoWithData<T>),
    WARNData(WarningWithData<T>),
}
impl<T> OkDataResponse<T> {
    /// Creates a new *Informational* response object with additional data for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    /// - `data`: `T` - Additional data to be returned to the frontend
    ///
    /// # Example:
    /// ```
    /// use rs_response::{ResponseWithData, OkDataResponse};
    ///
    /// fn some_fn() -> ResponseWithData<bool> {
    ///   Ok(
    ///     OkDataResponse::new_info(
    ///       "Test",
    ///       "This is an INFO result",
    ///       true,
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_info(category: impl Into<String>, message: impl Into<String>, data: T) -> Self {
        Self::INFOData(InfoWithData {
            category: category.into(),
            message: message.into(),
            data,
        })
    }

    /// Creates a new *Warning* response object with additional data for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    /// - `data`: `T` - Additional data to return to the frontend
    /// - `cause`: `impl Into<String>` - Further details of what caused the warning
    /// - `source`: `impl Into<String>` - The path to where the warning occurred
    ///
    /// # Example:
    /// ```
    /// use rs_response::{ResponseWithData, OkDataResponse};
    ///
    /// fn some_fn() -> ResponseWithData<String> {
    ///   Ok(
    ///     OkDataResponse::new_warning(
    ///       "Test",
    ///       "This is a WARNING result",
    ///       String::from("Hello World"),
    ///       "Caused for example purposes",
    ///       "rs_response::example::path()"
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_warning<Str: Into<String>>(
        category: impl Into<String>,
        message: impl Into<String>,
        data: T,
        cause: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self::WARNData(WarningWithData {
            category: category.into(),
            message: message.into(),
            data,
            cause: cause.into(),
            source: source.into(),
        })
    }

    /// - `drop_data` - Drops the additional data, converting the `OkDataResponse` to an `OkResponse`
    ///
    /// # Example:
    /// ```
    /// use rs_response::{Response, OkResponse, OkDataResponse};
    ///
    /// fn some_fn() -> Response {
    ///   let data_response = OkDataResponse::new_info(
    ///     "Example",
    ///     "This is an example response",
    ///     String::from("This is additional data"),
    ///   );
    ///
    ///   let response_without_data = data_response.drop_data();
    ///
    ///   Ok(response_without_data)
    /// }
    /// ```
    pub fn drop_data(self) -> OkResponse {
        match self {
            Self::INFOData(current) => OkResponse::INFO(Info {
                category: current.category,
                message: current.message,
            }),
            Self::WARNData(current) => OkResponse::WARN(Warning {
                category: current.category,
                message: current.message,
                cause: current.cause,
                source: current.source,
            }),
        }
    }
}
