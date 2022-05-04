use log::{error, info};
use rdkafka::{
  error::KafkaError,
  producer::{FutureProducer, FutureRecord},
  ClientConfig,
};
use std::{env, error::Error, time::Duration};

pub fn new_kafka_producer() -> FutureProducer {
  let kafka_bootstrap_servers =
    env::var("kafkaBootstrapServers").expect("env kafkaBootstrapServers error");
  let producer = ClientConfig::new()
    .set("bootstrap.servers", kafka_bootstrap_servers)
    .create()
    .expect("Producer creation error");
  producer
}

pub async fn publish(
  msg: &String,
  topic: &String,
  producer: &FutureProducer,
) -> Result<(), KafkaError> {
  println!("publish inter{:?}", msg);
  //
  let futures = (0..1)
    .map(|i| async move {
      let delivery_status = producer
        .send(
          FutureRecord {
            topic,
            partition: None,
            payload: Some(msg),
            key: Some(&format!("Key {}", i)),
            timestamp: None,
            headers: None,
          },
          Duration::from_secs(0),
        )
        .await;

      // This will be executed when the result is received.
      info!("Delivery status for message {} received", i);
      delivery_status
    })
    .collect::<Vec<_>>();

  // This loop will wait until all delivery statuses have been received.
  for future in futures {
    info!("Future completed. Result: {:?}", future.await);
  }
  Ok(())
}
