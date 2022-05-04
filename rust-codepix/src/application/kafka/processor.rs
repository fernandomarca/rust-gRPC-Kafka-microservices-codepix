use crate::{
  api_error::ApiError,
  application::{
    factory::{pixkey_usecase_factory, transaction_usecase_factory},
    model::{parse_json, to_json},
    usecase::transaction::TransactionUseCase,
  },
  domain::model::transaction::TransactionDto,
};
use log::{info, warn};
use rdkafka::{
  config::RDKafkaLogLevel,
  consumer::{CommitMode, Consumer, ConsumerContext, Rebalance, StreamConsumer},
  error::KafkaResult,
  message::BorrowedMessage,
  producer::FutureProducer,
  ClientConfig, ClientContext, Message, TopicPartitionList,
};

use super::producer::publish;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
struct CustomContext;

impl ClientContext for CustomContext {}

impl ConsumerContext for CustomContext {
  fn pre_rebalance(&self, rebalance: &Rebalance) {
    info!("Pre rebalance {:?}", rebalance);
  }

  fn post_rebalance(&self, rebalance: &Rebalance) {
    info!("Post rebalance {:?}", rebalance);
  }

  fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
    info!("Committing offsets: {:?}", result);
  }
}

// A type alias with your custom consumer can be created for convenience.
type TesteConsumer = StreamConsumer<CustomContext>;

pub struct KafkaProcessor {
  producer: FutureProducer,
}

impl KafkaProcessor {
  pub fn new(producer: FutureProducer) -> KafkaProcessor {
    KafkaProcessor { producer }
  }
  async fn process_message(&self, msg: &BorrowedMessage<'_>) -> Result<(), ApiError> {
    let topic = Message::topic(msg);
    let result = match topic {
      "transactions" => self.process_transaction(msg).await,
      "transaction_confirmation" => self.process_transaction_confirmation(msg).await,
      _ => {
        warn!("not a valid topic: {:?}", msg.payload_view::<str>());
        Err(ApiError::new(404, String::from("not a valid topic")))
      }
    };
    result
  }

  async fn process_transaction(&self, msg: &BorrowedMessage<'_>) -> Result<(), ApiError> {
    println!("process_message inter{:?}", msg);
    //
    let transaction_dto = parse_json(msg);
    let transaction_usecase = transaction_usecase_factory();
    let created_transaction = transaction_usecase.register(
      transaction_dto.account_from_id,
      transaction_dto.amount,
      transaction_dto.pix_key_key,
      transaction_dto.description,
      None,
    )?;
    println!("created_transaction {:?}", created_transaction);
    // relations in one query
    let pix_key_usecase = pixkey_usecase_factory();
    let pix_key = pix_key_usecase.find_pix_by_id(&created_transaction.pix_key_id_to)?;
    // let topic = "bank" + created_transaction.PixKeyTo.Account.Bank.Code
    let topic = format!("bank{}", pix_key.bank.code);
    //
    let transaction_json = to_json(&created_transaction);
    publish(&transaction_json, &topic, &self.producer)
      .await
      .expect("error in publish process_transaction");
    Ok(())
  }

  async fn process_transaction_confirmation(
    &self,
    msg: &BorrowedMessage<'_>,
  ) -> Result<(), ApiError> {
    let transaction = parse_json(msg);
    let transaction_usecase = transaction_usecase_factory();
    //
    if transaction.status == "confirmed".to_string() {
      self
        .confirm_transaction(transaction, transaction_usecase)
        .await?;
    } else if transaction.status == "completed".to_string() {
      let _result = transaction_usecase.complete(transaction.id)?;
    }
    Ok(())
  }

  async fn confirm_transaction(
    &self,
    transaction: TransactionDto,
    transaction_usecase: TransactionUseCase,
  ) -> Result<(), ApiError> {
    let confirmed_transaction = transaction_usecase.confirm(transaction.id)?;
    // relations in one query
    let pix_key_usecase = pixkey_usecase_factory();
    let pix_key = pix_key_usecase.find_pix_by_id(&confirmed_transaction.pix_key_id_to)?;
    // topic := "bank" + confirmedTransaction.AccountFrom.Bank.Code
    let topic = format!("bank{}", pix_key.bank.code);
    let transaction_json = to_json(&confirmed_transaction);
    publish(&transaction_json, &topic, &self.producer)
      .await
      .expect("error in publish transaction_confirm");
    Ok(())
  }

  pub async fn consume(
    &self,
    topics: Vec<String>,
    group_id: String,
    bootstrap: String,
  ) -> Result<(), ApiError> {
    let context = CustomContext;
    //
    let consumer: TesteConsumer = ClientConfig::new()
      .set("bootstrap.servers", bootstrap)
      .set("group.id", group_id)
      //.set("enable.partition.eof", "false")
      //.set("session.timeout.ms", "6000")
      //.set("enable.auto.commit", "true")
      //.set("statistics.interval.ms", "30000")
      .set("auto.offset.reset", "latest")
      .set_log_level(RDKafkaLogLevel::Debug)
      .create_with_context(context)
      .expect("Consumer creation failed");
    //
    let topics = topics
      .iter()
      .enumerate()
      .map(|(_i, v)| v.as_str())
      .collect::<Vec<&str>>();
    //
    consumer
      .subscribe(&topics)
      .expect("Can't subscribe to specified topics");
    //
    loop {
      match consumer.recv().await {
        Err(e) => warn!("Kafka error: {}", e),
        Ok(m) => {
          let payload = match m.payload_view::<str>() {
            None => "",
            Some(Ok(s)) => s,
            Some(Err(e)) => {
              warn!("Error while deserializing message payload: {:?}", e);
              ""
            }
          };
          //log
          info!(
            " consume key: '{:?}', payload: '{}', topic: {}, partition: {}, offset: {}, timestamp: {:?}",
            m.key(),
            payload,
            m.topic(),
            m.partition(),
            m.offset(),
            m.timestamp()
          );
          //
          // if let Some(headers) = m.headers() {
          //   for header in headers.iter() {
          //     info!("  Header {:#?}: {:?}", header.key, header.value);
          //   }
          // }
          //
          self.process_message(&m).await?;
          consumer.commit_message(&m, CommitMode::Async).unwrap();
        }
      };
    }
  }
}
