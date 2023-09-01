use rs_response::ErrorRepsonse;

pub fn create_error(
    message: impl Into<String>,
    details: impl Into<String>,
    source: &str,
) -> ErrorRepsonse {
    ErrorRepsonse::new_error(
        "Database",
        message,
        details,
        String::from("rs_db::") + source,
    )
}
