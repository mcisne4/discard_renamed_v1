use rs_response::Response;
mod dev_rs_db;

fn main() {
    eprintln!("=========================");
    eprintln!("=== RUNNING: 'rs_dev' ===");
    eprintln!("=========================\n");

    dev_rs_db::dev_rs_db(true)
}

pub fn print_response(title: &str, results: Response) {
    eprintln!("** DEV TEST: '{}' **", title);
    match results {
        Ok(data) => eprintln!("> OK:\n{:#?}\n", data),
        Err(err) => eprintln!("> ERR:\n{:#?}\n", err),
    }
}
