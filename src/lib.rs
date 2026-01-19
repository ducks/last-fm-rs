/// Last.fm API client library for Rust
///
/// Supports authentication and scrobbling for desktop applications.
mod auth;
mod auth_mode;
mod client;
mod error;
mod scrobble;
mod signature;
mod track;

pub use auth::{AuthToken, SessionKey};
pub use client::Client;
pub use error::{Error, Result};
pub use scrobble::{NowPlaying, Scrobble, ScrobbleResponse};
pub use track::{TrackInfo, TrackInfoResponse};
