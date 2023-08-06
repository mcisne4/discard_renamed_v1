fn db_connect() {
    match rs_db::DB::Settings.connect() {
        Ok(_) => eprintln!("\nSUCCESSFULLY CONNECTED TO 'settings' DATABASE"),
        Err(e) => eprintln!("\n{:#?}\nERROR CONNECTING TO 'settings' DATABASE", e),
    }
}

fn main() {
    println!("Hello, world!");

    db_connect();
}
