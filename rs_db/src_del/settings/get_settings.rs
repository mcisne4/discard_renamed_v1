use super::Settings;
use crate::connection::DB;
use rs_response::Toast;
use rusqlite::Error;

struct Row {
    theme: String,
    show_welcome_screen: u8,
}
impl Row {
    pub fn to_settings(&self) -> Settings {
        Settings {
            theme: match self.theme.as_str() {
                "dark" => String::from("dark"),
                "light" => String::from("light"),
                _ => String::from("dark"),
            },
            show_welcome_screen: match self.show_welcome_screen {
                0 => false,
                1 => true,
                _ => true,
            },
        }
    }
}

pub fn read_settings() -> Result<Settings, Toast> {
    let db = DB::Settings.connect()?;

    let row = db.query_row("Select * FROM settings WHERE id = 'main';", [], |row| {
        let _id = row.get::<usize, String>(0)?;
        let theme = row.get::<usize, String>(1)?;
        let show_welcome_screen = row.get::<usize, u8>(2)?;

        Ok(Row {
            theme,
            show_welcome_screen,
        })
    });

    let found_settings = match row {
        Ok(row) => Some(row.to_settings()),
        Err(e) => match e {
            Error::QueryReturnedNoRows => None,
            _ => {
                return Err(Toast::new_error_toast(
                    "Database",
                    "Unable to read settings values from database",
                    Some(e.to_string().as_str()),
                ))
            }
        },
    };

    let settings = match found_settings {
        Some(settings) => settings,
        None => {
            let settings = Settings {
                theme: String::from("dark"),
                show_welcome_screen: true,
            };

            db.execute(settings.as_insert_statement().as_str(), [])
                .map_err(|e| {
                    Toast::new_error_toast(
                        "Database",
                        "Could not add 'Settings' to the database",
                        Some(e.to_string().as_str()),
                    )
                })?;

            settings
        }
    };

    Ok(settings)
}
