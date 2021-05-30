use serde::{Deserialize, Serialize};
use strum::AsRefStr;

use super::request::SeverityKind;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogError {
  SessionNonexistent,
  Unknown,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
  session_id: i64,
  creation_time: i64,
  service_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  event_id: i64,
  creation_time: i64,
  session_id: Session,
  msg: String,
  severity_kind: SeverityKind
}
