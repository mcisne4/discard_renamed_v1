mod err;
mod ok;
pub use err::ErrorRepsonse;
pub use ok::OkResponse;

/// Result for querying data without a structured response
pub type DataResponse<T> = std::result::Result<T, ErrorRepsonse>;

/// Result for querying data and returning a structured response for the frontend
pub type Response<T> = std::result::Result<OkResponse<T>, ErrorRepsonse>;

/// Result for querying data and returning a list of structured responses for the frontend
pub type ResponseVec<T> = std::result::Result<Vec<OkResponse<T>>, ErrorRepsonse>;
