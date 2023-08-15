use crate::connection::DB;
use rs_response::Toast;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Theme {
    DARK,
    Light,
}

#[derive(Debug, Serialize)]
pub struct SettingsDB {
    id: String,
    theme: Theme,
    show_display_settings: bool,
}
impl SettingsDB {
    /// Initialize the Settings database
    pub fn init() -> Result<(), Toast> {
        let db = DB::Settings.connect()?;

        db.execute(
            "CREATE TABLE IF NOT EXIST settings (
              id TEXT PRIMARY KEY,
              theme TEXT NOT NULL,
              show_welcome_screen INTEGER NOT NULL);",
            [],
        )
        .map_err(|e| {
            Toast::new_error_toast(
                "Database",
                "Could not create the settings table",
                Some(e.to_string().as_str()),
            )
        })?;

        Ok(())
    }

    /// Create a SettingsDB instance with default values
    pub fn default() -> Self {
        Self {
            id: String::from("main"),
            theme: Theme::DARK,
            show_display_settings: true,
        }
    }

    //
}

pub fn tw() {
    let init = SettingsDB::default();
}
