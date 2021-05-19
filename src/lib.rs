// If client is enabled then we use
#[cfg(feature = "client")]
pub mod client;


// Types of arguments for auth handlers
use serde::{Deserialize, Serialize};
use strum::AsRefStr;

#[derive(Clone, Debug, Serialize, Deserialize, AsRefStr)]
pub enum LogSeverity {
  Info,
  Warning,
  Error,
  Fatal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageProps {
  source: String,
  severity: LogSeverity,
  msg: String,
}
