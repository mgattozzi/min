use serde::{
  Deserialize,
  Serialize,
};
use std::{
  collections::HashMap,
  error::Error,
  fs,
  path::{
    Path,
    PathBuf,
  },
  process::Command,
};
use structopt::StructOpt;
use toml::Value;
use tracing::{
  error,
  info,
};
use tracing_subscriber::FmtSubscriber;

mod files;

#[derive(StructOpt, Debug)]
#[structopt(name = "min")]
pub enum Opt {
  /// Create a brand new min project
  New(New),
}

#[derive(StructOpt, Debug)]
pub struct New {
  path: PathBuf,
}

fn main() {
  if let Err(e) = run() {
    error!("{}", e)
  }
}

fn run() -> Result<(), Box<dyn Error>> {
  let subscriber = FmtSubscriber::new();
  tracing::subscriber::set_global_default(subscriber)?;
  let opt = Opt::from_args();

  match opt {
    Opt::New(new_opts) => {
      if new_opts.path.exists() {
        return Err(
          format!("The path '{}' already exists", new_opts.path.display())
            .as_str()
            .into(),
        );
      } else {
        let mut path = new_opts.path;
        info!("Creating new project at '{}'", path.display());
        let status = Command::new("cargo")
          .arg("new")
          .arg(path.as_os_str())
          .output()?
          .status;
        if !status.success() {
          error!("Failed to create a new Rust project with cargo");
          std::process::exit(status.code().unwrap_or(1));
        }
        info!("Created a new Rust project");
        info!("Setting up project for min");
        path.push("Cargo.toml");
        setup_deps(&path)?;
        info!("Updated Cargo.toml with necessary dependencies");
        path.pop();
        path.push("src");
        path.push("main.rs");
        files::main_rs(&path)?;
      }
    }
  }
  Ok(())
}

#[derive(Serialize, Deserialize)]
struct Manifest {
  // We don't care about this we just need to get the field
  package: HashMap<String, Value>,
  #[serde(serialize_with = "toml::ser::tables_last")]
  dependencies: HashMap<String, Value>,
}

fn setup_deps(path: &Path) -> Result<(), Box<dyn Error>> {
  let mut manifest = toml::from_slice::<Manifest>(&fs::read(&path)?)?;

  manifest.dependencies.insert("min-lib".into(), "0.1".into());
  manifest.dependencies.insert("tracing".into(), "0.1".into());
  manifest
    .dependencies
    .insert("tracing-subscriber".into(), "0.2".into());
  manifest
    .dependencies
    .insert("tracing-futures".into(), "0.2".into());
  manifest
    .dependencies
    .insert("serde_json".into(), "1.0".into());

  "serde = { version = '1.0', features = ['derive'] }"
    .parse::<Value>()?
    .as_table()
    .unwrap()
    .iter()
    .for_each(|(k, v)| {
      manifest.dependencies.insert(k.into(), v.to_owned());
    });

  "tokio = { version = '0.2', features = ['full'] }"
    .parse::<Value>()?
    .as_table()
    .unwrap()
    .iter()
    .for_each(|(k, v)| {
      manifest.dependencies.insert(k.into(), v.to_owned());
    });

  "diesel = { version = '1.4', features = ['postgres', 'sqlite', 'mysql'] }"
    .parse::<Value>()?
    .as_table()
    .unwrap()
    .iter()
    .for_each(|(k, v)| {
      manifest.dependencies.insert(k.into(), v.to_owned());
    });

  fs::write(&path, toml::to_string(&manifest)?)?;
  Ok(())
}
