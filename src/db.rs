use std::path::Path;
use rusqlite::{Connection, Error};

const DBPATH: &'static str = "mwlDB.db";

pub fn get() -> Result<Connection, Error> {
    Connection::open(Path::new(DBPATH))
}
