// use rusqlite::Connection;
// use std::env::current_dir;
use std::path::PathBuf;

pub fn connect(_data_dir: Option<PathBuf>) {
    // let app_data_dir = match data_dir {
    //     Some(dir) => dir.join("com.renamed.app").join("db"),
    //     None => {
    //         let current_dir = current_dir()
    //             .map_err(|e| create_error("Failed to get current directory", e.to_string()));
    //         //
    //         PathBuf::from("./").join("com.renamed.app").join("app")
    //     }
    // };

    // eprintln!("APP DATA DIR: {}", app_data_dir.display());

    // let db = Connection::open(app_data_dir);
    // match db {
    //     Ok(_conn) => eprintln!("Connected"),
    //     Err(_e) => eprintln!("Not Connected"),
    // }
}
