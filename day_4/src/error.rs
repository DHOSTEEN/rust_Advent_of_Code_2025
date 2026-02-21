use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum Day4Error {
    #[error("no line found from Grid")]
    NoLine,
    #[error("no input file found for Day 4")]
    NoFile(#[from] io::Error),
}