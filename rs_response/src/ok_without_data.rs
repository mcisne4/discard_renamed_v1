// use crate::OkDataResponse;
use crate::ok_with_data::{InfoWithData, OkDataResponse, WarningWithData};

#[derive(Debug, serde::Serialize)]
pub struct Info {
    pub category: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct Warning {
    pub category: String,
    pub message: String,
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
///
/// # WARN Properties:
/// - `category`: `String` - The response object category
/// - `message`: `String` - The main response message to displayed
/// - `cause`: `String` - Further details of what caused the warning
/// - `source`: `String` - The path to where the warning occurred
///
/// # Methods:
/// - `new_info` - Creates a new *Informational* response object
/// - `new_warning` - Creates a new *Warning* response object
/// - `add_data` - Converts an `OkResponse` to an `OkDataResponse`
///
/// # Example:
/// ```
/// use rs_response::{ResponseVec, OkResponse};
///
/// fn some_fn() -> ResponseVec {
///   let info = OkResponse::new_info(
///     "Test",
///     "This is an INFO result",
///   );
///
///   let warn = OkResponse::new_warning(
///     "Test",
///     "This is a WARNING result",
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
pub enum OkResponse {
    INFO(Info),
    WARN(Warning),
}
impl OkResponse {
    /// Creates a new *Informational* response object for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    ///
    /// # Example:
    /// ```
    /// use rs_response::{Response, OkResponse};
    ///
    /// fn some_fn() -> Response {
    ///   Ok(
    ///     OkResponse::new_info(
    ///       "Test",
    ///       "This is an INFO result",
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_info(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self::INFO(Info {
            category: category.into(),
            message: message.into(),
        })
    }

    /// Creates a new *Warning* response object for the frontend
    ///
    /// # Arguments:
    /// - `category`: `impl Into<String>` - The response object category
    /// - `message`: `impl Into<String>` - The main response message to display
    /// - `cause`: `impl Into<String>` - Further details of what caused the warning
    /// - `source`: `impl Into<String>` - The path to where the warning occurred
    ///
    /// # Example:
    /// ```
    /// fn some_fn() -> Response {
    ///   Ok(
    ///     OkResponse::new_warning(
    ///       "Test",
    ///       "This is a WARNING result",
    ///       "Caused for example purposes",
    ///       "rs_response::example::path()"
    ///     )
    ///   )
    /// }
    /// ```
    pub fn new_warning(
        category: impl Into<String>,
        message: impl Into<String>,
        cause: impl Into<String>,
        source: impl Into<String>,
    ) -> Self {
        Self::WARN(Warning {
            category: category.into(),
            message: message.into(),
            cause: cause.into(),
            source: source.into(),
        })
    }

    /// Converts an `OkResponse` to an `OkDataResponse`
    ///
    /// # Arguments:
    /// - `data`: `T` - Additional data to return to the frontend
    pub fn add_data<T>(self, data: T) -> OkDataResponse<T> {
        match self {
            Self::INFO(current) => OkDataResponse::INFOData(InfoWithData {
                category: current.category,
                message: current.message,
                data,
            }),
            Self::WARN(current) => OkDataResponse::WARNData(WarningWithData {
                category: current.category,
                message: current.message,
                data,
                cause: current.cause,
                source: current.source,
            }),
        }
    }
}
