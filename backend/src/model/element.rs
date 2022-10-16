#![allow(dead_code)]

use std::time::Duration;
use diesel::{
    connection::SimpleConnection, prelude::*, r2d2::ConnectionManager,
    Insertable, RunQueryDsl, SqliteConnection,
};
use r2d2::Pool;
use strum_macros::Display;

use crate::model;


// region: Types
#[derive(Debug, Clone, Default, Queryable, Insertable)]
#[diesel(table_name = list)]
pub struct ListElement {
    pub id: i32,
    pub title: String,
    pub notes: Option<String>,
    pub tags: Option<String>,
    pub status: String,
    pub ctime: String, 
    pub mtime: Option<String>,
}

#[derive(Debug, Clone, Default, Insertable, AsChangeset)]
#[diesel(table_name = list)]
pub struct ListElementPatch {
    pub title: Option<String>,
    pub notes: Option<String>,
    pub tags: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum ListElementStatus {
    Open,
    Archived,
    Closed,
}

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type LETuple = (i32,String,Option<String>,Option<String>,String,Option<String>,String,);
// endregion: Types

// region: Diesel Connection Options
#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}
// endregion: Diesel Connection Options

pub struct ListElementMAC;

diesel::table! {
    list {
        id -> Integer,
        title ->  Text,
        notes -> Nullable<Text>,
        tags -> Nullable<Text>,
        ctime -> Text,
        mtime -> Nullable<Text>,
        status -> Text,
    }
}

// LE Model Access Controller
impl ListElementMAC {
    pub fn create(db: &DbPool, patch: ListElementPatch) -> Result<ListElement, model::Error> {
        use crate::model::element::list::dsl::list;

        let row: LETuple = diesel::insert_into(list)
            .values(&patch)
            .get_result(&mut db.clone().get().unwrap())?;

        Ok(parse_get_result(row))
    }

    pub fn get_all(db: &DbPool, limit: Option<i64>) -> Result<Vec<ListElement>, model::Error> {
        use crate::model::element::list::dsl::list;
       
        let rows: Vec<LETuple>;
        if limit.is_none() {
            rows = list.load(&mut db.clone().get().unwrap())?;
        }
        else{
            rows = list.limit(limit.unwrap()).load(&mut db.clone().get().unwrap())?;
        }

        Ok(rows.iter().map(|row| parse_get_result(row.clone())).collect())
    }

    pub fn get_from_name(db: &DbPool, name: String) -> Result<Vec<ListElement>, model::Error> {
        use crate::model::element::list::dsl::list;
        use crate::model::element::list::*;

        let rows: Vec<LETuple> = list.filter(title.like(format!("%{}%",name))).load(&mut db.clone().get().unwrap())?;

        Ok(rows.iter().map(|row| parse_get_result(row.clone())).collect())       
    } 

    pub fn get_from_tags(db: &DbPool, tag_string: String) -> Result<Vec<ListElement>, model::Error> {
        // handlie in parseing, when you add a tag, whitspace is replaced by dashes 
        // and when you parse a search the terms a first split by white space for words 
        // then the white space in words replaced by dahses
        
        use crate::model::element::list::dsl::list;
        use crate::model::element::list::*;

        let rows: Vec<LETuple> = list.filter(tags.like(format!("%{}%", tag_string))).load(&mut db.clone().get().unwrap())?;

        Ok(rows.iter().map(|row| parse_get_result(row.clone())).collect())       
    }

    pub fn update(db: &DbPool, le_title: String, patch: &ListElementPatch) -> Result<ListElement, model::Error> {
        
        use crate::model::element::list::dsl::list;
        use crate::model::element::list::*;

        let mut patch = patch.clone();

        if patch.title.is_none() {
            patch.title = Some(le_title.clone());
        }

        let row : LETuple = diesel::update(list).filter(title.eq(le_title)).set(patch).get_result(&mut db.clone().get().unwrap())?;
        
        Ok(parse_get_result(row))   
    }

    pub fn delete(db: &DbPool, le_title: String) -> Result<(), model::Error> {
        
        use crate::model::element::list::dsl::list;
        use crate::model::element::list::*;

        diesel::delete(list.filter(title.eq(le_title.clone())))
            .execute(&mut db.clone().get().unwrap())
            .expect(format!("Error Deleting Post: {}", le_title).as_str());
        
        Ok(())
    }

    pub fn create_from_path(db_path: &str, patch: ListElementPatch) -> Result<ListElement, model::Error> {
        let conn = establish_connection(db_path)?;

        Self::create(&conn, patch)
    }

    pub fn get_all_from_path(db_path: &str, limit: Option<i64>) -> Result<Vec<ListElement>, model::Error> {
        let conn = establish_connection(db_path)?;

        Self::get_all(&conn, limit)
    }

    pub fn get_from_name_from_path(db_path: &str, name: String) -> Result<Vec<ListElement>, model::Error> {
        let conn = establish_connection(db_path)?;

        Self::get_from_name(&conn, name)
    }

    pub fn get_from_tags_from_path(db_path: &str, tag_string: String) -> Result<Vec<ListElement>, model::Error> {
        let conn = establish_connection(db_path)?;

        Self::get_from_tags(&conn, tag_string)
    }

    pub fn update_from_path(db_path: &str, le_title: String, patch: &ListElementPatch) -> Result<ListElement, model::Error> {
        let conn = establish_connection(db_path)?;

        Self::update(&conn, le_title, patch)
    }

    pub fn delete_from_path(db_path: &str, le_title: String) -> Result<(), model::Error> {
        let conn = establish_connection(db_path)?;

        Self::delete(&conn, le_title)
    }
}

fn establish_connection(db_path: &str) -> Result<DbPool, model::Error> {
    Ok(Pool::builder()
        .max_size(5)
        .connection_customizer(Box::new(ConnectionOptions {
            enable_wal: true,
            enable_foreign_keys: true,
            busy_timeout: Some(Duration::from_secs(30)),
        }))
        .build(ConnectionManager::<SqliteConnection>::new(db_path))
        .unwrap())
}

fn parse_get_result(row: LETuple) -> ListElement {
    ListElement {
        id: row.0,
        title: row.1,
        notes: row.2,
        tags: row.3,
        ctime: row.4,
        mtime: row.5,
        status: row.6,
    }
}

// region: Tests
#[cfg(test)]
#[path = "../_tests/model_list_element.rs"]
mod tests;
// endregion: Tests
