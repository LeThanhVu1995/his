use actix_web::{web, HttpResponse};
use uuid::Uuid;
use crate::infrastructure::repositories::template_repo::TemplateRepo;
use crate::domain::{services::{render_svc::RenderSvc, send_svc::SendSvc}, entities::message::Message};

pub async fn send(
    db: web::Data<sqlx::Pool<sqlx::Postgres>>,
    body: web::Json<crate::dto::message_dto::SendMessageReq>,
) -> actix_web::Result<HttpResponse> {
    let (channel, to) = (&body.channel, &body.to);

    // nếu có template thì render
    let (subject, content, tmpl_code) = if let Some(code) = &body.template_code {
        let t = TemplateRepo { db: &db }.get_by_code(code).await
            .map_err(|_| crate::error::AppError::Internal("DB".into()))?
            .ok_or(crate::error::AppError::BadRequest("template_not_found".into()))?;

        let html = RenderSvc::new().render(&t.body, &body.variables.clone().unwrap_or(serde_json::json!({})))
            .map_err(|_| crate::error::AppError::BadRequest("render".into()))?;

        (t.subject, html, Some(t.code))
    } else {
        (body.subject.clone(), body.body.clone(), None)
    };

    let msg = Message {
        id: Uuid::new_v4(),
        template_code: tmpl_code,
        channel: channel.clone(),
        to_addr: to.clone(),
        subject,
        body: content,
        status: "QUEUED".into(),
        err: None,
        created_at: chrono::Utc::now(),
        sent_at: None,
    };

    let id = SendSvc { db: &db }.enqueue_and_send(msg).await
               .map_err(|e| {
                   tracing::warn!(?e, "send fail");
                   crate::error::AppError::Internal("send".to_string())
               })?;

    Ok(HttpResponse::Created().json(crate::dto::message_dto::SendMessageRes { id }))
}
