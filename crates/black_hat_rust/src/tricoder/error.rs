use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Reqwest error: {0}")]
  Reqwest(String),
}

impl std::convert::From<reqwest::Error> for Error {
  fn from(err: reqwest::Error) -> Self {
    Error::Reqwest(err.to_string())
  }
}
