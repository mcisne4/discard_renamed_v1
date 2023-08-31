use super::util::DB_DISPLAY_NAME;
use crate::errors::DbError;
use rs_response::DataResponse;

/// A 'Settings' object to return to the frontend
///
/// **NOTE:** The `Settings` object should be created
/// from a `SettingsDB` item using the `to_settings`
/// method
///
/// # Example:
/// ```
/// use rs_db::settings_db::{Settings, SettingsDB};
/// use rs_response::{ResponseWithData, OkDataResponse};
///
/// fn get_settings() -> ResponseWithData<Settings> {
///   let default_settings = Settings::default();
///   let settings = default_settings.to_settings();
///
///   Ok(OkDataResponse::new_info(
///     "Example",
///     "These are the default settings",
///     settings,
///   ))
/// }
/// ```
#[derive(serde::Serialize)]
pub struct Settings {
    pub theme: String,
    pub show_welcome_screen: bool,
    pub show_db_notif: bool,
    pub show_confirm_notif: bool,
}

/// A data structure for interacting with *Settings*
/// data from the database.
///
/// **NOTE:** To return *Settings* data to the frontend,
/// convert the `SettingsDB` data to a `Settings` object
/// usng the `to_settings` method
///
/// # Properties:
/// - `theme`: `u8` - The UI theme used by the frontend
///   - Value: `0` - "DARK" theme *(default)*
///   - Value: `1` - "LIGHT" theme
/// - `show_welcome_screen`: `u8` - Whether to show the *Welcome* screen on startup
///   - Value: `0` - false
///   - Value: `1` - true *(default)*
/// - `show_db_notif`: `u8` - Whether to show notifications related to database changes
///   - Value: `0` - false *(default)*
///   - Value: `1` - true
/// - `show_confirm_notif`: `u8` - Whether to show the *Confirm* dialog when executing a rename
///   - Value: `0` - false
///   - Value: `1` - true *(default)*
///
/// # Methods:
/// - `new` - Create a new `SettingsDB` item from provided values
/// - `default` - Creates a new `SettingsDB` item with default values
/// - `parse` - Creates a new `SettingsDB` item from data from the frontend
/// - `to_settings` - Converts the `SettingsDB` data into a `Settings` object for the frontend
///
/// # Example:
/// ```
/// use rs_db::settings_db::{Settings, SettingsDB};
/// use rs_response::{ResponseWithData, OkDataResponse};
///
/// fn get_settings() -> ResponseWithData<Settings> {
///   let default_settings = Settings::default();
///   let settings = default_settings.to_settings();
///
///   Ok(OkDataResponse::new_info(
///     "Example",
///     "These are the default settings",
///     settings,
///   ))
/// }
/// ```
#[derive(Debug)]
pub struct SettingsDB {
    pub theme: u8,
    pub show_welcome_screen: u8,
    pub show_db_notif: u8,
    pub show_confirm_notif: u8,
}
impl SettingsDB {
    /// Create a new `SettingsDB` item from provided values
    ///
    /// # Arguments:
    /// - `theme`: `u8` - The UI theme used by the frontend
    ///   - Value: `0` - "DARK" theme
    ///   - Value: `1` - "LIGHT" theme
    /// - `show_welcome_screen`: `u8` - Whether to show the *Welcome* screen on startup
    ///   - Value: `0` - False
    ///   - Value: `1` - True
    /// - `show_db_notif`: `u8` - Whether to show notifications related to database changes
    ///   - Value: `0` - False
    ///   - Value: `1` - True
    /// - `show_confirm_notif`: `u8` - Whether to show the *Confirm* dialog when executing a rename
    ///   - Value: `0` - False
    ///   - Value: `1` - True
    ///
    /// # Example:
    /// ```
    /// use rs_db::settings_db::SettingsDB;
    ///
    /// fn my_custom_settings() -> SettingsDB {
    ///   let theme = 1_u8;
    ///   let show_welcome_screen: 0_u8;
    ///   let show_db_notif: 0_u8;
    ///   let show_confirm_notif: 0_u8;
    ///
    ///   SettingsDB::new(
    ///     theme,
    ///     show_welcome_screen,
    ///     show_db_notif,
    ///     show_confirm_notif,
    ///   )
    /// }
    /// ```
    pub fn new(
        theme: u8,
        show_welcome_screen: u8,
        show_db_notif: u8,
        show_confirm_notif: u8,
    ) -> Self {
        Self {
            theme,
            show_welcome_screen,
            show_db_notif,
            show_confirm_notif,
        }
    }

    /// Creates a new `SettingsDB` item with default values
    ///
    /// # Default Values:
    ///
    /// | Property              | Value  | Description                                   |
    /// | ----------------------| -------| --------------------------------------------- |
    /// | `theme`               | `0_u8` | Use the "DARK" theme                          |
    /// | `show_welcome_screen` | `1_u8` | Show the *Welcome* screen on startup          |
    /// | `show_db_notif`       | `0_u8` | Do not show database notifications            |
    /// | `show_confirm_notif`  | `1_u8` | Show *Confirm* dialog when executing a rename |
    ///
    /// # Example:
    /// ```
    /// use rs_db::settings_db::SettingsDB;
    /// use rs_response::DataResponse;
    ///
    /// fn get_defult_theme() -> &str {
    ///   let settings = SettingsDB::default();
    ///
    ///   match settings.theme {
    ///     0 => "Dark Theme",
    ///     _ => "Light Theme",
    ///   }
    /// }
    /// ```
    pub fn default() -> Self {
        Self {
            theme: 0,
            show_welcome_screen: 1,
            show_db_notif: 0,
            show_confirm_notif: 1,
        }
    }

    /// Creates a new `SettingsDB` item from data from the frontend
    ///
    /// # Arguments:
    /// - `theme`: `String` - The UI theme used by the frontend. Acceptable values are either `"DARK"` or `"LIGHT"`
    /// - `show_welcome_screen`: `bool` - Whether to show the *Welcome* screen on startup
    /// - `show_db_notif`: `bool` - Whether to show notifications related to database changes
    /// - `show_confirm_notif`: `bool` - Whether to show the *Confirm* dialog when executing a rename
    ///
    /// # Example:
    /// ```
    /// use rs_db::settings_db::SettingsDB;
    /// use rs_response::{OkResponse, ResponseVec};
    ///
    /// #[tauri::command]
    /// fn get_user_disabled_settings(
    ///   theme: String,
    ///   show_welcome_screen: bool,
    ///   show_db_notif: bool,
    ///   show_confirm_notif: bool,
    /// ) -> ResponseVec {
    ///   let settings = SettingsDB::parse(
    ///     theme,
    ///     show_welcome_screen,
    ///     show_db_notif,
    ///     show_confirm_notif,
    ///   );
    ///
    ///   let mut disabled_settings: Vec<OkResponse> = vec![];
    ///
    ///   if settings.show_welcome_screen == 0 {
    ///     disabled_settings.push(
    ///       OkResponse::new_info(
    ///         "Settings",
    ///         "The 'Welcome' screen on startup is disabled"
    ///       )
    ///     );
    ///   }
    ///
    ///   if settings.show_db_notif == 0 {
    ///     disabled_settings.push(
    ///       OkResponse::new_info(
    ///         "Settings",
    ///         "Database notifications are disabled"
    ///       )
    ///     );
    ///   }
    ///
    ///   if settings.show_confirm_notif == 0 {
    ///     disabled_settings.push(
    ///       OkResponse::new_info(
    ///         "Settings",
    ///         "The 'Confirm' notification on renaming is disabled"
    ///       )
    ///     );
    ///   }
    ///
    ///   Ok(disabled_settings)
    /// }
    /// ```
    pub fn parse(
        theme: String,
        show_welcome_screen: bool,
        show_db_notif: bool,
        show_confirm_notif: bool,
    ) -> DataResponse<SettingsDB> {
        let err_msg = format!("Could not parse the provided '{}' data", DB_DISPLAY_NAME);

        let theme = match theme.to_uppercase().as_str() {
            "DARK" => 0_u8,
            "LIGHT" => 1_u8,
            _ => {
                return Err(DbError::SettingsDb.create_error(
                    err_msg,
                    format!(
                        "'{}' is not a valid value for the 'theme'. The 
                    'theme' value should either be 'DARK' or 'LIGHT'",
                        theme
                    ),
                ))
            }
        };

        let show_welcome_screen = match show_welcome_screen {
            false => 0_u8,
            true => 1_u8,
        };

        let show_db_notif = match show_db_notif {
            false => 0_u8,
            true => 1_u8,
        };

        let show_confirm_notif = match show_confirm_notif {
            false => 0_u8,
            true => 1_u8,
        };

        Ok(Self {
            theme,
            show_welcome_screen,
            show_db_notif,
            show_confirm_notif,
        })
    }

    /// Converts the `SettingsDB` data into a `Settings` object for the frontend
    ///
    /// # Example:
    /// ```
    /// use rs_db::settings_db::{Settings, SettingsDB};
    /// use rs_response::{ResponseWithData, OkDataResponse};
    ///
    /// fn get_settings() -> ResponseWithData<Settings> {
    ///   let default_settings = Settings::default();
    ///   let settings = default_settings.to_settings();
    ///
    ///   Ok(OkDataResponse::new_info(
    ///     "Example",
    ///     "These are the default settings",
    ///     settings,
    ///   ))
    /// }
    /// ```
    pub fn to_settings(&self) -> Settings {
        Settings {
            theme: match self.theme {
                0 => String::from("DARK"),
                _ => String::from("LIGHT"),
            },
            show_welcome_screen: match self.show_welcome_screen {
                0 => false,
                _ => true,
            },
            show_db_notif: match self.show_db_notif {
                0 => false,
                _ => true,
            },
            show_confirm_notif: match self.show_confirm_notif {
                0 => false,
                _ => true,
            },
        }
    }
}
