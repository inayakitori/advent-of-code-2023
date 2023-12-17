use std::num::ParseIntError;
use thiserror::Error;
use regex::Regex;

pub trait SafeSplit{
    fn split_once_safe(&self, delimiter: &str) -> Result<(&str, &str), SplitError>;
}

impl SafeSplit for str{
    fn split_once_safe(&self, delimiter: &str) -> Result<(&str, &str), SplitError> {
        let delimiter_count = Regex::new(delimiter).unwrap().captures_len();

        return match delimiter_count{
            1 => self.split_once(delimiter).ok_or(SplitError{
                delimiter: delimiter.to_string(),
                delimiter_count: delimiter_count as u32,
            }),
            _ => Err(SplitError{
                delimiter: delimiter.to_string(),
                delimiter_count: delimiter_count as u32,
            })
        }

    }
}

#[derive(Error, Debug, Clone)]
pub enum ParseError{
    #[error("could not parse int")]
    ParseIntError(#[from] ParseIntError),
    #[error("{0}")]
    SplitError(#[from] SplitError),
    #[error("text {0} not found")]
    TextNotFoundError(String),
    #[error("Unknown Error")]
    Unknown
}

#[derive(Error, Debug, Clone)]
#[error("expected to get 1 delimiter {delimiter}, got {delimiter_count}")]
pub struct SplitError{
    delimiter: String,
    delimiter_count: u32,
}