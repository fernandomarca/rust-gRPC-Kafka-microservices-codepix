pub mod cmd;
use log::info;
use rdkafka::util::get_rdkafka_version;
use std::env;
use structopt::StructOpt;
use tokio::task;

use self::cmd::{Action, CommandLineArgs};
use crate::{
  api_error::ApiError,
  application::kafka::{processor::KafkaProcessor, producer::new_kafka_producer},
};
use crate::{
  application::grpc::pixgrpc::{server_grpc, MyPix},
  infrastructure::db::{self},
};

pub async fn execute() -> Result<(), Box<dyn std::error::Error>> {
  let CommandLineArgs { action, port } = CommandLineArgs::from_args();

  match action {
    Action::Start => {
      db::init();
      let pix_service = MyPix {};
      server_grpc(pix_service, port).await?;
    }
    Action::Kafka => {
      async fn start_kafka() -> Result<(), ApiError> {
        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);
        //
        let producer = new_kafka_producer();
        //topics
        let kafka_transaction_topic =
          env::var("kafkaTransactionTopic").expect("env kafkaTransactionTopic eror");
        let kafka_transaction_confirmation_topic = env::var("kafkaTransactionConfirmationTopic")
          .expect("env kafkaTransactionConfirmationTopic eror");
        let topics: Vec<String> = vec![
          kafka_transaction_topic,
          kafka_transaction_confirmation_topic,
        ];
        //envs
        let group_id = env::var("kafkaConsumerGroupId").expect("env kafkaConsumer eror");
        let bootstrap = env::var("kafkaBootstrapServers").expect("env kafkaBootstrapServers eror");
        //
        let kafka_processor = KafkaProcessor::new(producer);
        kafka_processor.consume(topics, group_id, bootstrap).await
      }
      let _result = start_kafka();
    }
    Action::All => {
      //start gRpc
      async fn start_grpc(port: String) -> Result<(), Box<dyn std::error::Error>> {
        db::init();
        let pix_service = MyPix {};
        server_grpc(pix_service, port).await?;
        Ok(())
      }
      // start kafka
      async fn start_kafka() -> Result<(), ApiError> {
        let (version_n, version_s) = get_rdkafka_version();
        info!("rd_kafka_version: {}, {}", version_n, version_s);
        let producer = new_kafka_producer();
        //topics
        let kafka_transaction_topic =
          env::var("kafkaTransactionTopic").expect("env kafkaTransactionTopic eror");
        let kafka_transaction_confirmation_topic = env::var("kafkaTransactionConfirmationTopic")
          .expect("env kafkaTransactionConfirmationTopic eror");
        let topics: Vec<String> = vec![
          kafka_transaction_topic,
          kafka_transaction_confirmation_topic,
        ];
        let group_id = env::var("kafkaConsumerGroupId").expect("env kafkaConsumer erRor");
        //envs
        let ambient = env::var("AMBIENT").expect("env ambient error");
        let bootstrap = if let true = ambient == "dev".to_string() {
          env::var("LOCALHOST_KAFKA").expect("env LOCALHOST_KAFKA error")
        } else {
          env::var("kafkaBootstrapServers").expect("env kafkaBootstrapServers error")
        };
        //
        let kafka_processor = KafkaProcessor::new(producer);

        kafka_processor.consume(topics, group_id, bootstrap).await?;
        Ok(())
      }

      let grpc = task::spawn(async move {
        let _result = start_grpc(port).await;
      });
      //
      let kafka = task::spawn(async move {
        let _result = start_kafka().await;
      });

      let _result = tokio::join!(grpc, kafka);
    }
  };
  Ok(())
}
