use rs_response::Toast;

pub enum DBErrors {
    CONNECTDataDir,
    CONNECTCreateDir(std::io::Error),
    CONNECTOpenConn {
        db_display_name: String,
        error: rusqlite::Error,
    },
    TABLEEXISTSQueryRow {
        db_display_name: String,
        error: rusqlite::Error,
    },
    GETCOLUMNSNoTable {
        db_display_name: String,
    },
    GETCOLUMNSPragmaStmt {
        db_name: String,
        error: rusqlite::Error,
    },
    GETCOLUMNSPragmaQuery {
        db_name: String,
        error: rusqlite::Error,
    },
    GETCOLUMNSRowNext {
        db_name: String,
        error: rusqlite::Error,
    },
    DROPTABLEExecute {
        db_display_name: String,
        error: rusqlite::Error,
    },
}
impl DBErrors {
    pub fn to_toast(&self) -> Toast {
        let (description, details): (String, String) = match self {
            DBErrors::CONNECTDataDir => (
                "Could not access the database directory".to_owned(),
                "The data directory path was not found".to_owned(),
            ),
            DBErrors::CONNECTCreateDir(err) => (
                "Could not access the database directory".to_owned(),
                err.to_string(),
            ),
            DBErrors::CONNECTOpenConn {
                db_display_name,
                error,
            } => (
                format!("Unable to open the '{}' database", db_display_name),
                error.to_string(),
            ),
            DBErrors::TABLEEXISTSQueryRow {
                db_display_name,
                error,
            } => (
                format!("Unable to check if the '{}' table exists", db_display_name),
                error.to_string(),
            ),
            DBErrors::GETCOLUMNSNoTable { db_display_name } => (
                format!("Could not read the '{}' database", db_display_name),
                format!("The '{}' database does not exist", db_display_name),
            ),
            DBErrors::GETCOLUMNSPragmaStmt { db_name, error } => (
                "Development Error".to_owned(),
                format!(
                    "Invalid pragma query statement for '{}':\n{}",
                    db_name,
                    error.to_string().as_str()
                ),
            ),
            DBErrors::GETCOLUMNSPragmaQuery { db_name, error } => (
                "Development Error".to_owned(),
                format!(
                    "Invalid pragma query map for '{}':\n{}",
                    db_name,
                    error.to_string().as_str()
                ),
            ),
            DBErrors::GETCOLUMNSRowNext { db_name, error } => (
                "Development Error".to_owned(),
                format!(
                    "Could not retrieve the next column in the '{}' table:\n{}",
                    db_name,
                    error.to_string().as_str()
                ),
            ),
            DBErrors::DROPTABLEExecute {
                db_display_name,
                error,
            } => (
                format!("Could not remove the '{}' table", db_display_name),
                error.to_string(),
            ),
        };

        Toast::new_error_toast("Database", description.as_str(), Some(details.as_str()))
    }
}
