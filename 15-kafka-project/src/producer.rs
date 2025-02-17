use std::time::Duration;

use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use rdkafka::ClientConfig;

pub fn create() -> FutureProducer {
    let mut config = ClientConfig::new();
    config.set("bootstrap.servers", "localhost:9092");

    let producer: FutureProducer = config.create().expect("Failure in creating producer");

    producer
}

pub async fn produce(future_producer: FutureProducer, msg: String) {
    let record = FutureRecord::to("test-topic")
        .payload(msg.as_str())
        .key("Test-Key");

    let status_delievery = future_producer
        .send(record, Timeout::After(Duration::from_secs(2)))
        .await;

    match status_delievery {
        Ok(report) => println!("Message Sent {:?}", report),
        Err(e) => println!("Error producing.. {:?}", e),
    }
}
