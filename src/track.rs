use serde::{Deserialize, Serialize};

/// Image with size variant
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
  #[serde(rename = "#text")]
  pub url: String,
  pub size: String,
}

/// Artist information (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
  pub name: String,
  #[serde(default)]
  pub mbid: String,
  pub url: String,
}

/// Album information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
  pub artist: String,
  pub title: String,
  #[serde(default)]
  pub mbid: String,
  pub url: String,
  #[serde(default)]
  pub image: Vec<Image>,
  #[serde(rename = "@attr", default)]
  pub attr: Option<AlbumAttr>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumAttr {
  pub position: String,
}

/// Tag information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
  pub name: String,
  pub url: String,
}

/// Top tags wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopTags {
  #[serde(default)]
  pub tag: Vec<Tag>,
}

/// Wiki content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wiki {
  pub published: String,
  pub summary: String,
  pub content: String,
}

/// Streamable information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Streamable {
  #[serde(rename = "#text")]
  pub text: String,
  pub fulltrack: String,
}

/// Track information from track.getInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
  pub name: String,
  #[serde(default)]
  pub mbid: String,
  pub url: String,
  #[serde(deserialize_with = "deserialize_optional_string_as_u64", default)]
  pub duration: Option<u64>,
  #[serde(default)]
  pub streamable: Option<Streamable>,
  #[serde(deserialize_with = "deserialize_string_as_u64", default)]
  pub listeners: u64,
  #[serde(deserialize_with = "deserialize_string_as_u64", default)]
  pub playcount: u64,
  pub artist: Artist,
  #[serde(default)]
  pub album: Option<Album>,
  #[serde(deserialize_with = "deserialize_optional_string_as_u64", default)]
  pub userplaycount: Option<u64>,
  #[serde(deserialize_with = "deserialize_optional_string_as_u64", default)]
  pub userloved: Option<u64>,
  #[serde(default)]
  pub toptags: Option<TopTags>,
  #[serde(default)]
  pub wiki: Option<Wiki>,
}

/// Response wrapper for track.getInfo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfoResponse {
  pub track: TrackInfo,
}

// Custom deserializers for Last.fm's string-encoded numbers
fn deserialize_string_as_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s: String = Deserialize::deserialize(deserializer)?;
  s.parse().unwrap_or(0).pipe(Ok)
}

fn deserialize_optional_string_as_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
  D: serde::Deserializer<'de>,
{
  let s: Option<String> = Deserialize::deserialize(deserializer)?;
  Ok(s.and_then(|s| s.parse().ok()))
}

// Helper trait for pipe syntax
trait Pipe: Sized {
  fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
    f(self)
  }
}

impl<T> Pipe for T {}
