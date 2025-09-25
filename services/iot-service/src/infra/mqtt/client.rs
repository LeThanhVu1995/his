use rumqttc::{Client, MqttOptions, QoS, EventLoop, Packet, Incoming};

pub struct Mqtt{ pub client: Client, pub evloop: EventLoop }
impl Mqtt{
  pub fn connect()->Self{
    let host=std::env::var("MQTT_BROKER_HOST").unwrap_or("localhost".into());
    let port: u16 = std::env::var("MQTT_BROKER_PORT").ok().and_then(|v|v.parse().ok()).unwrap_or(1883);
    let mut opts=MqttOptions::new(std::env::var("MQTT_CLIENT_ID").unwrap_or("his-iot".into()), host, port);
    let u=std::env::var("MQTT_USERNAME").ok(); let p=std::env::var("MQTT_PASSWORD").ok();
    if let (Some(u), Some(p))=(u,p){ opts.set_credentials(u,p); }
    opts.set_keep_alive(std::time::Duration::from_secs(30));
    let (client, evloop)=Client::new(opts, 10);
    Self{ client, evloop }
  }
}

pub async fn subscribe_vitals(cli:&mut Client){ let topic=crate::infra::mqtt::topics::vitals_topic(); let _=cli.subscribe(topic, QoS::AtLeastOnce); }
// iot-service src/infra/mqtt/client.rs placeholder
