pub fn vitals_topic()->String{ std::env::var("MQTT_TOPIC_VITALS").unwrap_or("iot/v1/+/vitals".into()) }
// iot-service src/infra/mqtt/topics.rs placeholder
