use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WordleError<T: PartialEq + Debug> {
    #[error("Max tries exceeded")]
    MaxTriesExceeded,
    #[error("The word `{0}` is not present in the word list")]
    InvalidWord(T),
    #[error("The word `{0}` is not the same length as the word to guess")]
    WordLengthMismatch(T),
    #[error("The word `{0}` has already been guessed")]
    WordAlreadyGuessed(T),
    #[error("unknown data store error")]
    Unknown,
}
