use reqwest::Client;

use super::{LogMessageProps, LogSeverity};

#[derive(Clone)]
pub struct LogService {
  client: Client,
  log_service_url: String,
  service_name: String,
}

impl LogService {
  pub fn new(log_service_url: &str, service_name: &str) -> Self {
    LogService {
      client: Client::new(),
      log_service_url: String::from(log_service_url),
      service_name: String::from(service_name),
    }
  }

  pub async fn log(&self, severity: LogSeverity, msg: &str) {
    let lmp = LogMessageProps {
      source: self.service_name.clone(),
      severity,
      msg: msg.to_owned(),
    };

    self
      .client
      .post(format!("{}/log_message", self.log_service_url))
      .json(&lmp)
      .send()
      .await;
  }

  pub async fn info(&self, msg: &str) {
    self.log(LogSeverity::Info, msg).await
  }
  pub async fn warning(&self, msg: &str) {
    self.log(LogSeverity::Warning, msg).await
  }
  pub async fn error(&self, msg: &str) {
    self.log(LogSeverity::Error, msg).await
  }
  pub async fn fatal(&self, msg: &str) {
    self.log(LogSeverity::Fatal, msg).await
  }
}
