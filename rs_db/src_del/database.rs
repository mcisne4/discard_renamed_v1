mod errors;
use errors::DBErrors;

use rs_response::Toast;
use rusqlite::Connection;
use std::fs;

pub enum DB {
    Settings,
}
impl DB {
    fn name(&self) -> String {
        match self {
            DB::Settings => String::from("settings"),
        }
    }

    fn display_name(&self) -> String {
        match self {
            DB::Settings => String::from("Settings"),
        }
    }

    pub fn connect(&self) -> Result<Connection, Toast> {
        let db_dir = match dirs_next::data_dir() {
            Some(dir) => dir.join("com.renamed.app/db/"),
            None => {
                return Err(DBErrors::CONNECTDataDir.to_toast());
            }
        };

        fs::create_dir_all(db_dir.clone()).map_err(|e| DBErrors::CONNECTCreateDir(e).to_toast())?;

        let db_filename = self.name() + ".db";
        let db_path = db_dir.join(db_filename);

        let db = Connection::open(db_path).map_err(|e| {
            DBErrors::CONNECTOpenConn {
                db_display_name: self.display_name(),
                error: e,
            }
            .to_toast()
        })?;

        Ok(db)
    }

    pub fn table_exists(&self) -> Result<bool, Toast> {
        let db = self.connect()?;

        let exists = db
            .query_row(
                "SELECT EXISTS (SELECT name FROM sqlite_schema WHERE type='table' AND name=?1);",
                [self.name()],
                |row| {
                    let val = row.get::<usize, usize>(0)?;
                    Ok(val)
                },
            )
            .map_err(|e| {
                DBErrors::TABLEEXISTSQueryRow {
                    db_display_name: self.display_name(),
                    error: e,
                }
                .to_toast()
            })?;

        match exists {
            0 => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn get_columns(&self) -> Result<Vec<(String, String)>, Toast> {
        let db = self.connect()?;

        if !self.table_exists()? {
            return Err(DBErrors::GETCOLUMNSNoTable {
                db_display_name: self.display_name(),
            }
            .to_toast());
        }

        let mut pragma_stmt = db
            .prepare("SELECT * FROM pragma_table_info(?1);")
            .map_err(|e| {
                DBErrors::GETCOLUMNSPragmaStmt {
                    db_name: self.name(),
                    error: e,
                }
                .to_toast()
            })?;

        let mut rows = pragma_stmt.query([self.name()]).map_err(|e| {
            DBErrors::GETCOLUMNSPragmaQuery {
                db_name: self.name(),
                error: e,
            }
            .to_toast()
        })?;

        let mut cols_name_and_type: Vec<(String, String)> = vec![];
        while let Some(row) = rows.next().map_err(|e| {
            DBErrors::GETCOLUMNSRowNext {
                db_name: self.name(),
                error: e,
            }
            .to_toast()
        })? {
            match row.get::<usize, String>(1) {
                Err(_) => (),
                Ok(col_name) => match row.get::<usize, String>(2) {
                    Err(_) => (),
                    Ok(col_type) => cols_name_and_type.push((col_name, col_type)),
                },
            }
        }

        Ok(cols_name_and_type)
    }

    pub fn drop_table(&self) -> Result<(), Toast> {
        let db = self.connect()?;

        db.execute(
            format!("DROP TABLE IF EXISTS {};", self.name()).as_str(),
            [],
        )
        .map_err(|e| {
            DBErrors::DROPTABLEExecute {
                db_display_name: self.display_name(),
                error: e,
            }
            .to_toast()
        })?;

        Ok(())
    }

    //
}
