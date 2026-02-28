use std::{io, num::ParseIntError, str::Utf8Error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day6Error {
    #[error("No input file found for Day 6")]
    NoFile(#[from] io::Error),
    #[error("failed to parse number in colum")]
    NumParse(#[from] ParseIntError),
    #[error("failed to find suitable symbol")]
    NoSymbol,
    #[error("byte chunks failed to parse to string")]
    ByteError(#[from] Utf8Error),
}
