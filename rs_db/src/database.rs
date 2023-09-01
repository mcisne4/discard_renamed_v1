use crate::error_factory::create_error;
use dirs_next;
use rs_response::DataResponse;
use rusqlite::Connection;
use std::fs;

const ERR_SRC: &str = "rs_db::connection::connect()";

pub fn connect(db_display_name: &str, db_name: &str) -> DataResponse<Connection> {
    let db_dir = match dirs_next::data_dir() {
        None => {
            return Err(create_error(
                "Could not access the database directory",
                "The data directory path was not found",
                ERR_SRC,
            ));
        }
        Some(data_dir) => {
            let db_dir = data_dir.join("com.renamed.app/db/");

            fs::create_dir_all(db_dir.clone()).map_err(|e| {
                create_error(
                    "Could not access the database directory",
                    e.to_string(),
                    ERR_SRC,
                )
            })?;

            db_dir
        }
    };

    let mut db_filename = db_name.to_string();
    if !db_filename.ends_with(".db") {
        db_filename += ".db"
    }

    let db_path = db_dir.join(db_filename);

    let db = Connection::open(db_path).map_err(|e| {
        create_error(
            format!("Unable to open the '{}' database", db_display_name),
            e.to_string(),
            ERR_SRC,
        )
    })?;

    Ok(db)
}
