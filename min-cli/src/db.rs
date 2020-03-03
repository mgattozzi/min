pub enum DB {
  Postgresql,
  MySql,
  Sqlite,
}

pub fn setup_db(path: &Path, prod_db: DB) -> Reuslt<(), Box<dyn Error>> {
  Ok(())
}
