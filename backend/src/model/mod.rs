mod db;
mod element; 

//re-export
pub use element::{ ListElementMAC, ListElementStatus, ListElement};
pub use db::init_db;

#[derive(thiserror::Error, Debug)]
#[allow(dead_code)]
pub enum Error {

    #[error("Entity Not Found - {0}[{1}] ")]
	EntityNotFound(&'static str, String),

    #[error(transparent)]
    RusqliteError(#[from] rusqlite::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

    #[error(transparent)]
    DieselConnectionError(#[from] diesel::ConnectionError),

    #[error(transparent)]
    DieselResultError(#[from] diesel::result::Error),

}