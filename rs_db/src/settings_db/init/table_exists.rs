use super::super::settings::{DB_DISPLAY_NAME, DB_NAME};
use crate::error_factory::create_error;
use rs_response::DataResponse;
use rusqlite::Connection;

const ERR_SRC: &str = "settings_db::init::table_exists()";

pub fn settings_table_exists(db: &Connection) -> DataResponse<bool> {
    let sqlite_stmt = format!("SELECT * FROM pragma_table_list WHERE name = '{}'", DB_NAME);

    let mut stmt = db.prepare(&sqlite_stmt).map_err(|e| {
        create_error(
            "DEVELOPER ERROR: Invalid database query",
            e.to_string(),
            ERR_SRC,
        )
    })?;

    let exists = stmt.exists([]).map_err(|e| {
        create_error(
            format!("Could not query the '{}' database", DB_DISPLAY_NAME),
            e.to_string(),
            ERR_SRC,
        )
    })?;

    Ok(exists)
}
