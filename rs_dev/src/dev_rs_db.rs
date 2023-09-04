use super::print_response;

pub fn dev_rs_db(run: bool) {
    if run == false {
        return;
    }

    eprintln!("=== TESTING: 'rs_db' ===\n");

    let data = rs_db::settings_db::initialize_settings_db();
    print_response("Settings Initialization", data);

    eprintln!("==========");
}
