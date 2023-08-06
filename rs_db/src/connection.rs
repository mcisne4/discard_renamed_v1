use rs_response::Toast;
use rusqlite::Connection;
use std::fs;

pub enum DB {
    Settings,
}
impl DB {
    pub fn connect(&self) -> Result<Connection, Toast> {
        let db_dir = match dirs_next::data_dir() {
            Some(dir) => dir.join("com.renamed.app/db/"),
            None => {
                return Err(Toast::new_error_toast(
                    "Database",
                    "Database path could not be established",
                    Some("Could not determine the data directory path"),
                ));
            }
        };

        fs::create_dir_all(db_dir.clone()).map_err(|e| {
            Toast::new_error_toast(
                "Database",
                "Unable to access the database directory",
                Some(e.to_string().as_str()),
            )
        })?;

        let db_name = match self {
            DB::Settings => "settings.db",
        };

        let db_path = db_dir.join(db_name);

        let db = Connection::open(db_path).map_err(|e| {
            Toast::new_error_toast(
                "Database",
                format!("Unable to open the '{}' database", db_name).as_str(),
                Some(e.to_string().as_str()),
            )
        })?;

        Ok(db)
    }
}
