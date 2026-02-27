use std::{io, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day6Error {
    #[error("No input file found for Day 6")]
    NoFile(#[from] io::Error),
    #[error("failed to parse number in colum")]
    NumParse(#[from] ParseIntError),
    #[error("failed to find suitable symbol")]
    NoSymbol,
}
