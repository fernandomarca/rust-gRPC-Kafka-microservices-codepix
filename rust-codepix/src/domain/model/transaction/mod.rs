#[allow(dead_code)]
mod transaction_test;
use super::account::AccountModel;
use super::pix_key::PixKeyModel;
use crate::api_error::ApiError;
use crate::infrastructure::db::schema::transaction;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{self, Deserialize};
use std::error::Error;
use uuid::Uuid;
pub trait TransactionRepositoryInterface {
  fn register(&self, transaction: TransactionDto) -> Result<(), Box<dyn Error>>;
  fn save(&self, transaction: TransactionDto) -> Result<TransactionModel, ApiError>;
  fn find_by_id(&self, id: String) -> Result<TransactionModel, ApiError>;
}
pub trait TransactionActions {
  fn complete(&mut self);
  fn confirm(&mut self);
  fn cancel(&mut self, description: String);
}

#[derive(Debug, Queryable, Identifiable, Clone, Associations, AsChangeset, Deserialize)]
#[belongs_to(AccountModel, foreign_key = "account_from_id")]
#[belongs_to(PixKeyModel, foreign_key = "pix_key_id_to")]
#[table_name = "transaction"]
pub struct TransactionModel {
  pub id: String,
  pub amount: BigDecimal,
  pub status: String,
  pub description: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  // relations
  pub account_from_id: String,
  // relations
  pub pix_key_id_to: String,
  pub pix_key_kind: String,
  pub pix_key_key: String,
}
#[derive(Debug, Deserialize)]
pub struct TransactionDomainModel {
  #[serde(flatten)]
  pub db: TransactionModel,
  pub account_from: AccountModel,
  pub pix_key_to: PixKeyModel,
}
#[derive(Debug, Insertable)]
#[table_name = "transaction"]
pub struct TransactionDto {
  pub id: String,
  pub amount: BigDecimal,
  pub status: String,
  pub description: String,
  pub account_from_id: String,
  pub pix_key_id_to: Option<String>,
  pub pix_key_kind: String,
  pub pix_key_key: String,
}

impl TransactionDto {
  pub fn new(
    id: Option<String>,
    amount: BigDecimal,
    description: String,
    pix_key: PixKeyModel,
    account_from_id: String,
  ) -> Result<TransactionDto, &'static str> {
    let verify_id = if let Some(id) = id {
      id
    } else {
      Uuid::new_v4().to_string()
    };
    //check accounts
    let account_id_in_pix_key = pix_key.account_id;
    //
    let transaction = TransactionDto {
      id: verify_id,
      amount,
      status: "pending".to_string(),
      description,
      account_from_id,
      pix_key_id_to: Some(pix_key.id),
      pix_key_kind: pix_key.kind,
      pix_key_key: pix_key.key,
    };
    transaction.transaction_is_valid(account_id_in_pix_key)?;
    Ok(transaction)
  }
  /// Set the transaction status.
  pub fn set_status(&mut self, status: String) {
    self.status = status;
  }
  //valid transaction?
  fn transaction_is_valid(&self, account_id_in_pix_key: String) -> Result<(), &'static str> {
    if self.amount <= BigDecimal::from(0) {
      return Err("the amount must be greater than 0");
    }
    if self.status != "pending".to_string()
      && self.status != "completed".to_string()
      && self.status != "error".to_string()
      && self.status != "confirmed".to_string()
    {
      return Err("the status must be pending, completed, confirmed or error");
    }
    if self.account_from_id == account_id_in_pix_key {
      return Err("the account from and account to must be different");
    }
    Ok(())
  }
}

impl TransactionActions for TransactionDto {
  fn complete(&mut self) {
    self.set_status(String::from("completed"));
  }

  fn confirm(&mut self) {
    self.set_status(String::from("confirmed"));
  }

  fn cancel(&mut self, description: String) {
    self.set_status(String::from("error"));
    self.description = description;
  }
}

impl TransactionActions for TransactionModel {
  fn complete(&mut self) {
    self.status = String::from("completed");
  }

  fn confirm(&mut self) {
    self.status = String::from("confirmed");
  }

  fn cancel(&mut self, description: String) {
    self.status = String::from("error");
    self.description = description;
  }
}
