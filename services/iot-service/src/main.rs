mod config; mod telemetry; mod error; mod infra; mod domain; mod http; mod security;
use app_web::service_main;
use rumqttc::{Event, Packet};

#[actix_web::main]
async fn main()->std::io::Result<()> {
  let cfg = config::Settings::load();
  telemetry::init_tracing(&cfg);
  let pool=sqlx::postgres::PgPoolOptions::new().max_connections(16).connect(&cfg.database_url).await.expect("db");

  // MQTT worker
  tokio::spawn({ let db=pool.clone(); async move {
    let mut mqtt=crate::infra::mqtt::client::Mqtt::connect();
    crate::infra::mqtt::client::subscribe_vitals(&mut mqtt.client).await;
    loop {
      match mqtt.evloop.poll().await { Ok(ev) => if let Event::Incoming(pkt)=ev { match pkt { Packet::Publish(p) => {
            let topic=String::from_utf8_lossy(&p.topic).to_string();
            let parts: Vec<_>=topic.split('/').collect();
            let device_code=parts.get(2).map(|s|*s).unwrap_or("");
            if let Ok(txt)=std::str::from_utf8(&p.payload){ if let Ok(json)=serde_json::from_str::<serde_json::Value>(txt){ let svc=crate::domain::services::ingest_svc::IngestSvc{ db:&db }; let _=svc.ingest_vital_json(device_code, &json).await; } }
          }, _=>{} } }, Err(e)=>{ tracing::warn!(?e,"mqtt poll"); tokio::time::sleep(std::time::Duration::from_secs(1)).await; }
      }
    }
  }});

  service_main!(
    service_name: cfg.service_name.clone(),
    config: cfg,
    permission_catalog: crate::security::policy::permission_catalog,
    set_permissions_registered: crate::http::handlers::health::set_permissions_registered,
    configure_app: |cfg: &mut actix_web::web::ServiceConfig| { cfg.configure(http::mount); },
    validator: app_auth::KeycloakValidator::from_security_config(&config::Settings::load().security)
  )
}
