mod table_exists;
mod validate_table;

use super::settings::{DB_DISPLAY_NAME, DB_NAME};
use crate::database;
// use crate::error_factory::create_error;
use rs_response::{DataResponse, OkResponse, Response};
use table_exists::settings_table_exists;
use validate_table::validate_table_columns;

// const ERR_SRC: &str = "settings_db::init::initialize_settings_db()";

pub fn initialize_settings_db() -> Response {
    let db = database::connect(DB_DISPLAY_NAME, DB_NAME)?;

    let exists = settings_table_exists(&db)?;
    eprintln!("- TABLE EXISTS: {}", exists);

    if exists {
        let valid_table = validate_table_columns(&db)?;
        eprintln!("  - VALID TABLE COLUMNS: {}", valid_table);
    }

    eprintln!("");

    Ok(OkResponse::new_info(
        "Database",
        "The database has been initialized",
    ))
}

// pub fn table_exists(db: &rusqlite::Connection) -> DataResponse<()> {
//     let mut stmt = db
//         .prepare("SELECT * FROM pragma_database_list")
//         .map_err(|e| {
//             create_error(
//                 format!("Could not query the '{}' database", DB_DISPLAY_NAME),
//                 e.to_string(),
//                 ERR_SRC,
//             )
//         })?;

//     let mut rows = stmt.query([]).map_err(|e| {
//         create_error(
//             "Could not query the 'settings' database",
//             e.to_string(),
//             ERR_SRC,
//         )
//     })?;

//     while let Some(row) = rows.next().map_err(|e| {
//         create_error(
//             "Could not query the 'settings' database",
//             e.to_string(),
//             ERR_SRC,
//         )
//     })? {
//         eprintln!("row: {:?}", row);
//         // let id = row.get::<usize, usize>(0).map_err(|e| {
//         //     create_error(
//         //         "Could not query the 'settings' database",
//         //         e.to_string(),
//         //         ERR_SRC,
//         //     )
//         // })?;

//         // let second = row
//         //     .get(1)
//         //     .map_err(|e| create_error("Could not get the 2nd item", e.to_string(), ERR_SRC))?;

//         // eprintln!("Column 0: {}", id);
//         // eprintln!("Column 1: {}", second);
//     }

//     Ok(())
// }
