// use rs_db::settings_db::SettingsDB;
// use rs_db::DB;
use rs_db::database::DB;
use rs_response::Toast;

pub fn dev_rs_db(run: bool) -> Result<(), Toast> {
    if run == false {
        return Ok(());
    }

    let exists = DB::Settings.table_exists()?;
    eprintln!("\nThe 'Settings' table exists?: {}", exists);

    let cols = DB::Settings.get_columns()?;
    eprintln!("\nColumns:");
    for (col_name, col_type) in cols {
        eprintln!("  {}: {}", col_name, col_type);
    }

    DB::Settings.drop_table()?;
    eprintln!("\nThe 'Settings' table has been dropped");

    Ok(())
}
