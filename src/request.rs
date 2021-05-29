// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogSeverityKind {
  Info,
  Warning,
  Error,
  Fatal,
}

impl TryFrom<u8> for LogSeverityKind {
  type Error = u8;
  fn try_from(val: u8) -> Result<LogSeverityKind, u8> {
    match val {
      x if x == LogSeverityKind::Info as u8 => Ok(LogSeverityKind::Info),
      x if x == LogSeverityKind::Warning as u8 => Ok(LogSeverityKind::Warning),
      x if x == LogSeverityKind::Error as u8 => Ok(LogSeverityKind::Error),
      x if x == LogSeverityKind::Fatal as u8 => Ok(LogSeverityKind::Fatal),
      x => Err(x),
    }
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessageProps {
  source: String,
  severity: LogSeverityKind,
  msg: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeartbeatProps {
  source: String,
}
