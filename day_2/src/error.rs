use thiserror::Error;
use std::num::ParseIntError;

#[derive(Debug, Error)]
pub enum Day2Error {
    #[error("failed to parse ID")]
    ParseID(#[from] ParseIntError),

    #[error("fancy regex failed")]
    FancyRegex(#[from] fancy_regex::Error),

    #[error("no start point from id string")]
    NoStart,
    #[error("no end point from id string")]
    NoEnd,

    #[error("thread paniced")]
    ThreadPanic,
}