// src/cli.rs
// use crate as we are not in `main.rs so no need `mod error;
use crate::error::AppError; 
use std::{env, path::PathBuf};


pub fn get_path_from_cli() -> Result<PathBuf, AppError> {
  // [0, 1, 2] `1` is the index to get the input param from cli (file path)
  match env::args().nth(1) {
    Some(arg) => Ok(arg.into()),
    None      => Err(AppError::Cli("missing file path".into())),
  }
}
