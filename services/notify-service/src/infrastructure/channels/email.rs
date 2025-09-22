use anyhow::Context;
use lettre::{Message, SmtpTransport, Transport};

pub async fn send(to: &str, subject: &str, body: &str) -> anyhow::Result<()> {
    let from = std::env::var("SMTP_FROM").unwrap_or_else(|_| "no-reply@his.local".into());
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().context("bad to")?)
        .subject(subject.to_string())
        .body(body.to_string())?;

    // dev: relay local mailhog
    let mailer = SmtpTransport::relay(&std::env::var("SMTP_HOST").unwrap_or_else(|_| "localhost".into()))?
        .build();

    mailer.send(&email).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(())
}
