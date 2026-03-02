use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Day7Error {
    #[error("No input file found for Day 6")]
    NoFile(#[from] io::Error),
    #[error("char not recognised in grid")]
    InvalidPoint,
    #[error("end of rows")]
    EOF,
}
