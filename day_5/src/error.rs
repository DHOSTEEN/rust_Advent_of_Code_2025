use thiserror::Error;
use std::io;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum Day5Error {
    #[error("no input file found for Day 5")]
    NoFile(#[from] io::Error),
    #[error("failed to parse id's")]
    IdParse(#[from] ParseIntError),
    #[error("uable to split on - for fresh is range")]
    RangeSplit,
}