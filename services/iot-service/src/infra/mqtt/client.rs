use rumqttc::{Client, MqttOptions, Connection, QoS};

pub struct Mqtt {
    pub client: Client,
    pub evloop: Connection,
}

impl Mqtt {
    pub fn connect() -> Self {
        let mut mqttoptions = MqttOptions::new("iot-service", "localhost", 1883);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(60));

        let (client, evloop) = Client::new(mqttoptions, 10);

        Self { client, evloop }
    }
}

pub async fn subscribe_vitals(client: &mut Client) {
    let _ = client.subscribe("iot/vitals/+", QoS::AtLeastOnce);
}
