use serde::{Deserialize, Serialize};

/// Authentication token (valid for 60 minutes)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
  pub token: String,
}

/// Session key (infinite lifetime until revoked)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionKey {
  pub key: String,
  pub name: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
  token: String,
}

#[derive(Debug, Deserialize)]
struct SessionResponse {
  session: SessionData,
}

#[derive(Debug, Deserialize)]
struct SessionData {
  name: String,
  key: String,
}

impl From<TokenResponse> for AuthToken {
  fn from(resp: TokenResponse) -> Self {
    AuthToken { token: resp.token }
  }
}

impl From<SessionResponse> for SessionKey {
  fn from(resp: SessionResponse) -> Self {
    SessionKey {
      key: resp.session.key,
      name: resp.session.name,
    }
  }
}
