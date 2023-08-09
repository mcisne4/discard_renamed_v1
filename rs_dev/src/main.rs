mod dev_rs_db;

fn main() {
    println!("Hello, world!");

    match dev_rs_db::dev_rs_db(true) {
        Ok(_) => eprintln!("'dev_rs_db': Completed Successfully\n"),
        Err(e) => eprintln!("'dev_rs_db': ERROR:\n{:#?}\n", e),
    }
}
