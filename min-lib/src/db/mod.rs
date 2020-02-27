use diesel::{
  connection::Connection,
  sqlite::SqliteConnection,
};
use std::error::Error;

pub fn setup_database() -> Result<(), Box<dyn Error>> {
  SqliteConnection::establish("dev-db")?;
  Ok(())
}
