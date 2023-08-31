use rs_response::ErrorRepsonse;

pub enum DbError {
    CONNECT,
    // SettingsGET,
    // SettingsINIT,
    SettingsDb,
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
                    Self::CONNECT => "connection::connect()",
                    // Self::SettingsINIT => "settings::init::init()",
                    // Self::SettingsGET => "settings::get_settings::get_settings()",
                    Self::SettingsDb => "settings_db::settings::SettingsDB",
                },
        )
    }
}
