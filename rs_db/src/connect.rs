use crate::errors::DbError;
use dirs_next;
use rs_response::DataResponse;
use rusqlite::Connection;
use std::fs;

pub fn connect(db_name: &str) -> DataResponse<Connection> {
    let db_dir = match dirs_next::data_dir() {
        None => {
            return Err(DbError::CONNECT.create_error(
                "Could not access the database directory",
                "The data directory path was not found",
            ))
        }
        Some(data_dir) => {
            let db_dir = data_dir.join("com.renamed.app/db/");

            fs::create_dir_all(db_dir.clone()).map_err(|e| {
                DbError::CONNECT
                    .create_error("Could not access the database directory", e.to_string())
            })?;

            db_dir
        }
    };

    let mut db_filename = db_name.clone().to_lowercase();
    if !db_filename.ends_with(".db") {
        db_filename += ".db"
    }

    let db_path = db_dir.join(db_filename);

    let db = Connection::open(db_path).map_err(|e| {
        DbError::CONNECT.create_error(
            format!("Unable to open the '{}' database", db_name),
            e.to_string(),
        )
    })?;

    Ok(db)
}
