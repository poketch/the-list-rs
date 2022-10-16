#![allow(dead_code)]

use crate::model;

use std::{fs::{self, File}, path::PathBuf, io::Read, thread};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;

// constants
const SQL_HOST: &str = "sql/db.tldb";

//sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/000-recreate-db.sql";

pub type Db = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;

pub fn init_db(db_path: Option<&str>) -> Result<Db, model::Error> {
    
    // -- Run the App sql files
    let app_db = new_db_pool(db_path.unwrap_or(SQL_HOST),1)?;
    let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
        .into_iter()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    paths.sort();

    // Execute each file
    for path in paths {
        if let Some(path) = path.to_str() {
            //only .sql and not the recreate
            if path.ends_with(".sql") {
                exec_sql(&app_db, &path)?;
            }
        }
    }

    //returning the app db location
    Ok(app_db)
}

fn  exec_sql(db: &Db, path: &str) -> Result<(), model::Error> {
    
    // read file
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //execute sql
    let pool = db.clone();
    match 
        thread::spawn( move || {
            let conn = pool.get().unwrap();
            conn.execute_batch(contents.as_str())
            .expect(format!("Error parsing SQL in exec_sql").as_str());
        }).join()
    {
        Ok(_) => (),
        Err(ex) => println!("WARNING: Error in exec_sql reading {}. Cause: {:?}", path, ex)
    };

    Ok(())
}

fn  new_db_pool(db: &str, max_conn: u32) -> Result<Db, model::Error> {
    let conn_string = format!("{}", db);

    Ok(
        Pool::builder()
        .max_size(max_conn)
        .build(SqliteConnectionManager::file(conn_string))
        .unwrap()
    )
}

// region: Test

#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;

// endregion: Test
