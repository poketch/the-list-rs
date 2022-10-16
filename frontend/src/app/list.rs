
use backend::{self, model::ListElement};
use std::{fmt::Display};
use chrono::{ NaiveDateTime, format::{DelayedFormat, StrftimeItems} };
use itertools::{Itertools};

#[derive(Debug)]
struct ListElementData<'a> { 
    title: String,
    notes: String,
    tags: Vec<String>,
    status: String,
    ctime: DelayedFormat<StrftimeItems<'a>>,
    mtime: DelayedFormat<StrftimeItems<'a>>,
}

impl ListElementData <'_> {

    fn new(le: ListElement) -> Self {
        Self {
            title: le.title,
            notes: le.notes.unwrap_or_default(),
            tags: le.tags.unwrap_or_default().split_whitespace().map(|tag| tag.to_string()).collect(),
            status: le.status,
            ctime: NaiveDateTime::parse_from_str(
                le.ctime.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .format("%H:%M %d-%m-%Y"),
            mtime: NaiveDateTime::parse_from_str(
                (le.mtime.unwrap_or(le.ctime)).as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .format("%H:%M %d-%m-%Y"),
        }
    }
}

impl Display for ListElementData <'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Title: {}, Notes: {}, Tags: {}, Status: {}, Created At: {}, Last Modified At: {}",
            self.title,
            self.notes,
            self.tags.iter().format(", "),
            self.status.to_string(),
            self.ctime,
            self.mtime,
        )
    }
}


#[derive(Debug)]
pub struct List <'a>{
    list: Vec<ListElementData <'a>>
}

impl List <'_>{

    pub fn new() -> Result<Self, backend::model::Error> {
        
        Ok(Self { 
            list:
                backend::model::ListElementMAC::get_all_from_path("backend/sql/db.tldb", None)?
                .iter().map( move |element|
                    ListElementData::new(element.to_owned())
                )
                .collect()
        })
    }
}

impl Display for List <'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for element in self.list.iter() {
            write!(f, "{}\n", element)?;
        }       

        Ok(())
    }
}