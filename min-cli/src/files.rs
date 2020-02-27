use quote::quote;
use std::{
  error::Error,
  fs,
  path::Path,
};

pub fn main_rs(path: &Path) -> Result<(), Box<dyn Error>> {
  fs::write(path, include_str!("../files/main.rs"))?;
  Ok(())
}
