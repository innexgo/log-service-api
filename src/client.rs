use reqwest::Client;

use super::request;

#[derive(Clone)]
pub struct LogService {
  client: Client,
  log_service_url: String,
  session_id: i64,
}

impl LogService {
  pub async fn new(
    log_service_url: &str,
    service_name: &str,
    version_major: i64,
    version_minor: i64,
    version_patch: i64,
  ) -> Self {
    let client = Client::new();
    let session: response::Event = client
      .post(format!("{}/session/new", log_service_url))
      .json(request::SessionNewProps {
        service_name: String::from(service_name),
        version_major,
        version_minor,
        version_patch,
      })
      .send()
      .await?
      .json();

    LogService {
      log_service_url: String::from(log_service_url),
      session_id: session.session_id,
      client,
    }
  }

  pub async fn log(&self, severity: request::SeverityKind, msg: &str) {
    let lmp = request::EventNewProps {
      session_id: self.session_id,
      msg: msg.to_owned(),
      severity,
    };

    self
      .client
      .post(format!("{}/log_message", self.log_service_url))
      .json(&lmp)
      .send()
      .await;
  }

  pub async fn info(&self, msg: &str) {
    self.log(SeverityKind::Info, msg).await
  }
  pub async fn warning(&self, msg: &str) {
    self.log(SeverityKind::Warning, msg).await
  }
  pub async fn error(&self, msg: &str) {
    self.log(SeverityKind::Error, msg).await
  }
  pub async fn fatal(&self, msg: &str) {
    self.log(SeverityKind::Fatal, msg).await
  }
}
