use std::{
  error::Error,
  fs,
  path::Path,
};

pub fn main_rs(path: &Path) -> Result<(), Box<dyn Error>> {
  fs::write(path, include_str!("../files/main.rs"))?;
  Ok(())
}

pub fn rustfmt_toml(path: &Path) -> Result<(), Box<dyn Error>> {
  fs::write(path, include_str!("../files/rustfmt.toml"))?;
  Ok(())
}

pub fn rust_toolchain(path: &Path) -> Result<(), Box<dyn Error>> {
  fs::write(path, include_str!("../files/rust-toolchain"))?;
  Ok(())
}
