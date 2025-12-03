use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
  #[error("HTTP request failed: {0}")]
  Http(#[from] reqwest::Error),

  #[error("JSON parsing failed: {0}")]
  Json(#[from] serde_json::Error),

  #[error("Last.fm API error: {0}")]
  Api(String),

  #[error("Authentication failed: {0}")]
  Auth(String),

  #[error("Invalid parameter: {0}")]
  InvalidParameter(String),

  #[error("URL parsing failed: {0}")]
  UrlParse(#[from] url::ParseError),
}
