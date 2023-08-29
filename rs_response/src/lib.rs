mod err;
mod ok_with_data;
mod ok_without_data;
pub use err::ErrorRepsonse;
pub use ok_with_data::OkDataResponse;
pub use ok_without_data::OkResponse;

/// Result type for performing an operation and returning data without a structured response
///
/// # Example:
/// ```
/// use rs_response::{DataResponse, ErrorResponse};
///
/// pub just_a_data_fn(num: usize) -> DataResponse<bool> {
///   match num {
///     0 => Ok(false),
///     1 => Ok(true),
///     _ => Err(ErrorResponse::new_error(
///       "Example",
///       "This is an ERROR response",
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     ))
///   }
/// }
/// ```
pub type DataResponse<T> = std::result::Result<T, ErrorRepsonse>;

/// Result type for performing an operation and returning a structured response for the frontend
///
/// **NOTE:** If data needs to be returned to the frontend, use `ResponseWithData` instead
///
/// # Example:
/// ```
/// use rs_response::{ErrorResponse, OkResponse, Response};
///
/// pub do_something(valid: bool) -> Response {
///   match valid {
///     true => Ok(OkResponse::new_info(
///       "Example",
///       "This is an INFO response",
///     )),
///
///     false => Err(ErrorResponse::new_error(
///       "Example",
///       "This is an ERROR response",
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     )),
///   }
/// }
/// ```
pub type Response = std::result::Result<OkResponse, ErrorRepsonse>;

/// Result type for performing an operation and returning a list of structured responses for the frontend
///
/// **NOTE:** If data needs to be returned to the frontend, use `ResponseVecWithData` instead
///
/// # Example:
/// ```
/// use rs_response::{ErrorResponse, OkResponse, ResponseVec};
///
/// pub do_things(a: usize, b: usize) -> ResponseVec {
///   let common_err = ErrorResponse::new_error(
///     "Common Error",
///     "This is an ERROR response",
///     "Caused for example purposes",
///     "rs_response::example::path()",
///   );
///  
///   let aa = match a {
///     0 => return Err(common_err),
///     1 => OkResponse::new_info(
///       "Example A",
///       "This is an INFO response",
///     ),
///     _ => OkResponse::new_warning(
///       "Example A",
///       "This is a WARNING response",
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     ),
///   }
///
///   let bb = match b {
///     0 => return Err(common_err),
///     1 => OkResponse::new_info(
///       "Example B",
///       "This is an INFO response",
///     ),
///     _ => OkResponse::new_warning(
///       "Example B",
///       "This is a WARNING response",
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     ),
///   }
///
///   Ok(vec![aa, bb])
/// }
/// ```
pub type ResponseVec = std::result::Result<Vec<OkResponse>, ErrorRepsonse>;

/// Result type for performing an operation and returning a structured response for the frontend
///
/// **NOTE:** If no data needs to be returned to the frontend, use `Response` instead
///
/// # Example:
/// ```
/// use rs_response::{ErrorResponse, OkDataResponse, ResponseWithData};
///
/// pub get_something(valid: bool) -> ResponseWithData<usize> {
///   match valid {
///     true => Ok(OkDataResponse::new_info(
///       "Example",
///       "This is an INFO response",
///       123456
///     )),
///
///     false => Err(ErrorResponse::new_error(
///       "Example",
///       "This is an ERROR response",
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     )),
///   }
/// }
/// ```
pub type ResponseWithData<T> = std::result::Result<OkDataResponse<T>, ErrorRepsonse>;

/// Result type for performing an operation and returning a list of structured responses for the frontend
///
/// **NOTE:** If no data needs to be returned to the frontend, use `ResponseVec` instead
///
/// Example:
/// ```
/// use rs_response::{ErrorResponse, OkDataResponse, ResponseVecWithData};
///
/// pub get_things(a: usize, b: usize) -> ResponseVecWithData<usize> {
///   let common_err = ErrorResponse::new_error(
///     "Common Error",
///     "This is an ERROR response",
///     "Caused for example purposes",
///     "rs_response::example::path()",
///   );
///  
///   let aa = match a {
///     0 => return Err(common_err),
///     1 => OkResponse::new_info(
///       "Example A",
///       "This is an INFO response",
///       123456,
///     ),
///     _ => OkResponse::new_warning(
///       "Example A",
///       "This is a WARNING response",
///       123456,
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     ),
///   }
///
///   let bb = match b {
///     0 => return Err(common_err),
///     1 => OkResponse::new_info(
///       "Example B",
///       "This is an INFO response",
///       789012,
///     ),
///     _ => OkResponse::new_warning(
///       "Example B",
///       "This is a WARNING response",
///       789012,
///       "Caused for example purposes",
///       "rs_response::example::path()",
///     ),
///   }
///
///   Ok(vec![aa, bb])
/// }
/// ```
pub type ResponseVecWithData<T> = std::result::Result<Vec<OkDataResponse<T>>, ErrorRepsonse>;
