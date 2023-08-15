use crate::connection::DB;
use rs_response::Toast;

pub fn init() -> Result<(), Toast> {
    let db = DB::Settings.connect()?;

    // TODO: Validate any existing tables

    db.execute(
        "CREATE TABLE IF NOT EXISTS settings  (
            id TEXT PRIMARY KEY,
            theme TEXT NOT NULL,
            show_welcome_screen INTEGER NOT NULL
            )",
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
