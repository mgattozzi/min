use std::{
  error::Error,
  fs,
  path::{
    Path,
    PathBuf,
  },
  process::Command,
  str::FromStr,
};
use tracing::{
  info,
  warn,
};

#[derive(Debug)]
pub enum DB {
  PostgreSQL,
  MySql,
  Sqlite,
}

impl FromStr for DB {
  type Err = Box<dyn Error>;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    Ok(match input {
      "postgres" => DB::PostgreSQL,
      "mysql" => DB::MySql,
      "sqlite" => DB::Sqlite,
      x => return Err(format!("'{}' is not a valid db.", x).as_str().into()),
    })
  }
}

pub fn setup_db(path: &mut PathBuf) -> Result<(), Box<dyn Error>> {
  match which::which("diesel") {
    Ok(_) => {
      info!("Found diesel_cli binary, checking that it's version is 1.4");
      let version = String::from_utf8(
        Command::new("diesel").arg("--version").output()?.stdout,
      )?;
      if version.trim() != "diesel 1.4.0" {
        warn!(
         "This version of min has only been tested with version 1.4 of the diesel_cli tool.\
          This may cause unexpected issues after setting up the database, but this likely will \
          not. If you want to avoid this issue run \
          `cargo install diesel_cli --version 1.4.0 --force`, then run the new command for min-cli \
          again"
        );
      } else {
        info!("diesel_cli is at version 1.4");
      }
    }
    Err(_) => {
      info!("diesel_cli binary was not found, installing now");
    }
  }

  info!("Using diesel_cli to setup dev and prod databases");
  path.push("migrations");
  fs::create_dir_all(&path)?;
  path.pop();
  setup_sqlite(path, "dev.db")?;
  setup_sqlite(path, "prod.db")?;

  Ok(())
}

fn setup_sqlite(path: &Path, db_name: &str) -> Result<(), Box<dyn Error>> {
  Command::new("diesel")
    .arg("database")
    .arg("setup")
    .arg("--database-url")
    .arg(db_name)
    .current_dir(path)
    .spawn()?
    .wait()?;
  Ok(())
}
