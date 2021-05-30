// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SeverityKind {
  Info,
  Warning,
  Error,
  Fatal,
}

impl TryFrom<u8> for SeverityKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<SeverityKind, u8> {
    match val {
      x if x == SeverityKind::Info as u8 => Ok(SeverityKind::Info),
      x if x == SeverityKind::Warning as u8 => Ok(SeverityKind::Warning),
      x if x == SeverityKind::Error as u8 => Ok(SeverityKind::Error),
      x if x == SeverityKind::Fatal as u8 => Ok(SeverityKind::Fatal),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionNewProps {
  pub service_name: String,
  pub version_major: i64,
  pub version_minor: i64,
  pub version_patch: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventNewProps {
  pub session_id: i64,
  pub msg: String,
  pub severity: SeverityKind,
}
