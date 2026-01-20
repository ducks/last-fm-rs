use serde::{Deserialize, Serialize};

/// "Now Playing" notification
#[derive(Debug, Clone, Serialize)]
pub struct NowPlaying {
  pub artist: String,
  pub track: String,
  pub album: Option<String>,
  pub track_number: Option<u32>,
  pub duration: Option<u64>,
  pub album_artist: Option<String>,
  pub player: Option<String>,
}

impl NowPlaying {
  pub fn new(artist: impl Into<String>, track: impl Into<String>) -> Self {
    Self {
      artist: artist.into(),
      track: track.into(),
      album: None,
      track_number: None,
      duration: None,
      album_artist: None,
      player: None,
    }
  }

  pub fn with_album(mut self, album: impl Into<String>) -> Self {
    self.album = Some(album.into());
    self
  }

  pub fn with_track_number(mut self, track_number: u32) -> Self {
    self.track_number = Some(track_number);
    self
  }

  pub fn with_duration(mut self, duration: u64) -> Self {
    self.duration = Some(duration);
    self
  }

  pub fn with_album_artist(mut self, album_artist: impl Into<String>) -> Self {
    self.album_artist = Some(album_artist.into());
    self
  }

  pub fn with_player(mut self, player: impl Into<String>) -> Self {
    self.player = Some(player.into());
    self
  }
}

/// Scrobble submission
#[derive(Debug, Clone, Serialize)]
pub struct Scrobble {
  pub artist: String,
  pub track: String,
  pub timestamp: u64,
  pub album: Option<String>,
  pub track_number: Option<u32>,
  pub duration: Option<u64>,
  pub album_artist: Option<String>,
  pub player: Option<String>,
}

impl Scrobble {
  pub fn new(
    artist: impl Into<String>,
    track: impl Into<String>,
    timestamp: u64,
  ) -> Self {
    Self {
      artist: artist.into(),
      track: track.into(),
      timestamp,
      album: None,
      track_number: None,
      duration: None,
      album_artist: None,
      player: None,
    }
  }

  pub fn with_album(mut self, album: impl Into<String>) -> Self {
    self.album = Some(album.into());
    self
  }

  pub fn with_track_number(mut self, track_number: u32) -> Self {
    self.track_number = Some(track_number);
    self
  }

  pub fn with_duration(mut self, duration: u64) -> Self {
    self.duration = Some(duration);
    self
  }

  pub fn with_album_artist(mut self, album_artist: impl Into<String>) -> Self {
    self.album_artist = Some(album_artist.into());
    self
  }

  pub fn with_player(mut self, player: impl Into<String>) -> Self {
    self.player = Some(player.into());
    self
  }
}

/// Scrobble response
#[derive(Debug, Deserialize)]
pub struct ScrobbleResponse {
  pub scrobbles: ScrobbleData,
}

#[derive(Debug, Deserialize)]
pub struct ScrobbleData {
  #[serde(rename = "@attr")]
  pub attr: ScrobbleAttr,
}

#[derive(Debug, Deserialize)]
pub struct ScrobbleAttr {
  pub accepted: u32,
  pub ignored: u32,
}
