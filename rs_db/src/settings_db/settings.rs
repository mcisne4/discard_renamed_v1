use crate::database;
use crate::error_factory::create_error;
use rs_response::DataResponse;
use serde::ser::{Serialize, SerializeStruct};

const ERR_SRC: &str = "settings_db::settings::Settings";
const ERR_SRC2: &str = "settings_db::settings::SettingsTable";
const DB_NAME: &str = "settings";
const DB_DISPLAY_NAME: &str = "Settings";
const DB_ID: &str = "'main'";

/// Data structure for interacting with the 'Settings' database and the frontend
///
/// **NOTE:** While properties are private, the data can be serialized when
/// sending it to the frontend
///
/// # Properties *(private)*:
/// - `theme`: `String` - The current UI theme. Acceptable values are `"DARK"` and `"LIGHT"`
/// - `welcome_screen`: `bool` - Whether to show the *Welcome* screen on startup
/// - `db_notifs`: `bool` - Whether to show database notifications
/// - `confirm_rename`: `bool` - Whether to show the *Confirm Rename* dialog on renaming
///
/// # Methods:
/// - `default_settings` - Creates a `Settings` item with default values
/// - `parse_settings` - Creates a `Settings` item from parsed data from the frontend
/// - `write` - Writes the `Settings` data to the database
/// - `read` - Reads `Settings` data from the database if available. Otherwise, writes default
/// values to the database and returns it
///
/// # Example:
/// ```
/// use rs_db::settings_db::Settings;
/// use rs_response::DataResponse;
///
/// fn set_default_settings() -> DataResponse<()> {
///   let default_settings = Settings::default_settings();
///
///   default_settings.write()?;
///
///   Ok(())
/// }
/// ```
pub struct Settings {
    theme: String,
    welcome_screen: bool,
    db_notifs: bool,
    confirm_rename: bool,
}
impl Serialize for Settings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Settings", 4)?;

        state.serialize_field("theme", &self.theme)?;
        state.serialize_field("welcome_screen", &self.welcome_screen)?;
        state.serialize_field("db_notifs", &self.db_notifs)?;
        state.serialize_field("confirm_rename", &self.confirm_rename)?;

        state.end()
    }
}
impl Settings {
    /// Creates a new `Settings` item
    ///
    /// **NOTE:** The data is not validated to ensure it conforms
    /// to the required format. Use the `default_settings` and
    /// `parse_settings` methods to create a validated `Settings`
    /// item
    ///
    /// **NOTE:** The `Settings` data is not written to the database
    ///
    /// # Arguments:
    /// - `theme`: `impl Into<String>` - The current UI theme. Acceptable values are `"DARK"` and `"LIGHT"`
    /// - `welcome_screen`: `bool` - Whether to show the *Welcome* screen on startup
    /// - `db_notifs`: `bool` - Whether to show database notifications
    /// - `confirm_rename`: `bool` - Whether to show the *Confirm Rename* dialog on renaming
    fn new(
        theme: impl Into<String>,
        welcome_screen: bool,
        db_notifs: bool,
        confirm_rename: bool,
    ) -> Self {
        Self {
            theme: theme.into(),
            welcome_screen,
            db_notifs,
            confirm_rename,
        }
    }

    /// Creates a `Settings` item from parsed data from the frontend
    ///
    /// **NOTE:** The `Settings` data is not written to the database
    ///
    /// # Defautl Values:
    ///
    /// | Property         | Value    | Description                                  |
    /// | ---------------- | -------- | -------------------------------------------- |
    /// | `theme`          | `"DARK"` | Use the "DARK" theme                         |
    /// | `welcome_screen` | `true`   | Show the *Welcome* screen on startup         |
    /// | `db_notifs`      | `false`  | Do not show database notifications           |
    /// | `confirm_rename` | `true`   | Show the *Confirm Rename* dialog on renaming |
    ///
    /// # Example:
    /// ```
    /// use rs_db::settings_db::Settings;
    /// use rs_response::DataResponse;
    ///
    /// fn reset_settings() -> DataResponse<Settings> {
    ///   let default_settings = Settings::default_settings();
    ///
    ///   default_settings.write()?;
    ///
    ///   Ok(default_settings)
    /// }
    /// ```
    pub fn default_settings() -> Self {
        Self {
            theme: String::from("DARK"),
            welcome_screen: true,
            db_notifs: false,
            confirm_rename: true,
        }
    }

    /// Creates a `Settings` item from parsed data from the frontend
    ///
    /// **NOTE:** The `Settings` data is not written to the database
    ///
    /// # Arguments:
    /// - `theme`: `impl Into<String>` - The current UI theme. Acceptable values are `"DARK"` and `"LIGHT"`
    /// - `welcome_screen`: `bool` - Whether to show the *Welcome* screen on startup
    /// - `db_notifs`: `bool` - Whether to show database notifications
    /// - `confirm_rename`: `bool` - Whether to show the *Confirm Rename* dialog on renaming
    ///
    /// # Example:
    /// ```
    /// use rs_db::Settings;
    /// use rs_response::{OkResponse, Response};
    ///
    /// #[tauri::command]
    /// fn update_settings(
    ///   theme: String,
    ///   welcome_screen: bool,
    ///   db_notifs: bool,
    ///   confirm_rename: bool
    /// ) -> Response {
    ///   let settings = Settings::parse_settings(
    ///     theme,
    ///     welcome_screen,
    ///     db_notifs,
    ///     confirm_rename,
    ///   )?;
    ///
    ///   settings.write()?;
    ///
    ///   Ok(
    ///     OkResponse::new_info(
    ///       "Database",
    ///       "The 'Settings' have been reset"
    ///     )
    ///   )
    /// }
    /// ```
    pub fn parse_settings(
        theme: impl Into<String>,
        welcome_screen: bool,
        db_notifs: bool,
        confirm_rename: bool,
    ) -> DataResponse<Self> {
        let message = format!("Invalid data was provided for '{}'", DB_DISPLAY_NAME);

        let theme: String = theme.into();
        let theme = theme.to_uppercase();
        let theme = match theme.as_str() {
            "DARK" => theme,
            "LIGHT" => theme,
            _ => {
                return Err(create_error(
                    message,
                    format!("'{}' is not a valid value for 'theme'", theme),
                    ERR_SRC,
                ))
            }
        };

        Ok(Self::new(theme, welcome_screen, db_notifs, confirm_rename))
    }

    /// Writes the `Settings` data to the database
    ///
    /// **NOTE:** Data is validated prior to being written
    /// to the database
    pub fn write(&self) -> DataResponse<()> {
        let db = database::connect(DB_DISPLAY_NAME, DB_NAME)?;

        let settings_table = SettingsTable::from_settings(self)?;

        let sqlite_stmt = format!(
            "INSERT or REPLACE INTO {} (
                id,
                theme,
                welcome_screen,
                db_notifs,
                confirm_rename
            ) VALUES (
                {},
                {},
                {},
                {},
                {}
            );",
            DB_NAME,
            DB_ID,
            settings_table.theme,
            settings_table.welcome_screen,
            settings_table.db_notifs,
            settings_table.confirm_rename
        );

        db.execute(&sqlite_stmt, []).map_err(|e| {
            create_error(
                format!("Could not write '{}' data to the database", DB_DISPLAY_NAME),
                e.to_string(),
                ERR_SRC,
            )
        })?;

        Ok(())
    }

    /// Reads `Settings` data from the database
    ///
    /// **NOTE:** If no previous `Settings` data is found, default
    /// values will be written to the database and retrieved
    ///
    /// **NOTE:** Data from the database is validated to ensure
    /// proper formatting
    ///
    /// # Example:
    /// ```
    /// use rs_db::Settings;
    /// use rs_response::DataResponse;
    ///
    /// fn get_settings() -> DataResponse<Settings> {
    ///   let settings = Settings::read()?;
    ///
    ///   Ok(settings)
    /// }
    /// ```
    pub fn read() -> DataResponse<Self> {
        let db = database::connect(DB_DISPLAY_NAME, DB_NAME)?;

        let sqlite_stmt = format!("SELECT * FROM {} WHERE id = {};", DB_DISPLAY_NAME, DB_ID);

        let row = db.query_row(&sqlite_stmt, [], |row| {
            let _id = row.get::<usize, String>(0)?;
            let theme = row.get::<usize, u8>(1)?;
            let welcome_screen = row.get::<usize, u8>(2)?;
            let db_notifs = row.get::<usize, u8>(3)?;
            let confirm_rename = row.get::<usize, u8>(4)?;

            Ok(SettingsTable::new(
                theme,
                welcome_screen,
                db_notifs,
                confirm_rename,
            ))
        });

        let settings = match row {
            Ok(settings_table) => settings_table.to_settings()?,
            Err(e) => match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    let defaults = Self::default_settings();
                    defaults.write()?;
                    defaults
                }
                _ => {
                    return Err(create_error(
                        format!(
                            "Unable to read values from the '{}' database",
                            DB_DISPLAY_NAME
                        ),
                        e.to_string(),
                        ERR_SRC,
                    ))
                }
            },
        };

        Ok(settings)
    }
}

/// A data structure used to interact with the 'settings' database
///
/// # Methods:
/// - `new` - Creates a new `SettingsTable` item
/// - `from_settings` - Converts `Settings` data to `SettingsTable` data
/// - `to_settings` - Converts `SettingsTable` data to `Settings` data
struct SettingsTable {
    theme: u8,
    welcome_screen: u8,
    db_notifs: u8,
    confirm_rename: u8,
}
impl SettingsTable {
    /// Creates a new `SettingsTable` item
    ///
    /// **NOTE:** The `SettingsTable` data is not validated
    pub fn new(theme: u8, welcome_screen: u8, db_notifs: u8, confirm_rename: u8) -> Self {
        Self {
            theme,
            welcome_screen,
            db_notifs,
            confirm_rename,
        }
    }

    /// Converts `Settings` data to `SettingsTable` data
    ///
    /// **NOTE:** Data is validated to ensure proper formatting
    pub fn from_settings(settings: &Settings) -> DataResponse<Self> {
        Ok(Self {
            theme: match settings.theme.as_str() {
                "DARK" => 0,
                "LIGHT" => 1,
                _ => {
                    return Err(create_error(
                        format!(
                            "Unable to convert '{}' data to write to the database",
                            DB_DISPLAY_NAME
                        ),
                        format!("'{}' is not a valid 'theme' value", settings.theme),
                        ERR_SRC,
                    ))
                }
            },
            welcome_screen: match settings.welcome_screen {
                true => 1,
                false => 0,
            },
            db_notifs: match settings.db_notifs {
                true => 1,
                false => 0,
            },
            confirm_rename: match settings.confirm_rename {
                true => 1,
                false => 0,
            },
        })
    }

    /// Converts `SettingsTable` data to `Settings` data
    ///
    /// **NOTE:** Data is validated to ensure proper formatting
    pub fn to_settings(&self) -> DataResponse<Settings> {
        fn validate_0_or_1<T>(
            key: &str,
            value: u8,
            if_value_is_0: T,
            if_value_is_1: T,
        ) -> DataResponse<T> {
            match value {
                0 => Ok(if_value_is_0),
                1 => Ok(if_value_is_1),
                _ => Err(create_error(
                    format!(
                        "Unable to parse the '{}' data from the database due to invalid data",
                        DB_DISPLAY_NAME
                    ),
                    format!(
                        "The '{}' value should be either 0 or 1, but found {}",
                        key, value
                    ),
                    ERR_SRC2,
                )),
            }
        }

        Settings::parse_settings(
            validate_0_or_1("theme", self.theme, "DARK", "LIGHT")?,
            validate_0_or_1("welcome_screen", self.welcome_screen, false, true)?,
            validate_0_or_1("db_notifs", self.db_notifs, false, true)?,
            validate_0_or_1("confirm_rename", self.confirm_rename, false, true)?,
        )
    }
}
