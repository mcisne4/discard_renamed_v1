use rs_response::ErrorRepsonse;

pub enum DbError {
    CONNECT,
}
impl DbError {
    pub fn create_error(
        &self,
        message: impl Into<String>,
        details: impl Into<String>,
    ) -> ErrorRepsonse {
        ErrorRepsonse::new_error(
            "Database",
            message.into(),
            details.into(),
            String::from("rs_db::")
                + match self {
                    Self::CONNECT => "connect::connect()",
                },
        )
    }
}
