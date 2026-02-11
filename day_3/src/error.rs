use thiserror::Error;
use std::num::ParseIntError;
use std::io;

#[derive(Debug, Error)]
pub enum Day3Error {

    #[error("no such file")]
    NoFile(#[from] io::Error),

    #[error("digits in line failed to parse to u32")]
    ParseErr(#[from] ParseIntError),

    #[error("must be a digit 0-9")]
    NotDigit,

}