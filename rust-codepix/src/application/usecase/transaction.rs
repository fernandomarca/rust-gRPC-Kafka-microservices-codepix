use crate::api_error::ApiError;
use crate::diesel::RunQueryDsl;
use crate::domain::model::pix_key::PixKeyRepositoryInterface;
use crate::domain::model::transaction::TransactionActions;
use crate::domain::model::transaction::TransactionDto;
use crate::domain::model::transaction::TransactionModel;
use crate::domain::model::transaction::TransactionRepositoryInterface;
use crate::infrastructure::db::connection;
use crate::infrastructure::db::schema::transaction;
use bigdecimal::BigDecimal;
use diesel::*;
use std::marker::Send;
pub struct TransactionUseCase {
  pixkey_repo: Box<dyn PixKeyRepositoryInterface + Send>,
  transaction_repo: Box<dyn TransactionRepositoryInterface + Send>,
}

impl TransactionUseCase {
  pub fn register(
    &self,
    account_id: String,
    amount: BigDecimal,
    pix_key_to: String,
    description: String,
    id: Option<String>,
  ) -> Result<TransactionModel, ApiError> {
    //find account
    let account = self.pixkey_repo.find_account(&String::from(account_id))?;
    //find key by kind
    let pix_key = self.pixkey_repo.find_key_by_key(&pix_key_to)?;
    //new transaction and save
    let new_transaction =
      TransactionDto::new(id, amount, description, pix_key.pix.to_owned(), account.id)
        .map_err(|e| ApiError::new(400, e.to_string()))?;
    //
    let transaction = self.transaction_repo.save(new_transaction)?;
    Ok(transaction)
  }

  pub fn confirm(&self, transaction_id: String) -> Result<TransactionModel, ApiError> {
    let conn = connection()?;
    let result: TransactionModel = diesel::update(transaction::table)
      .filter(transaction::id.eq(transaction_id))
      .set(transaction::status.eq("confirmed"))
      .get_result(&conn)?;
    Ok(result)
  }

  pub fn complete(&self, transaction_id: String) -> Result<TransactionModel, ApiError> {
    let conn = connection()?;
    let result: TransactionModel = diesel::update(transaction::table)
      .filter(transaction::id.eq(transaction_id))
      .set(transaction::status.eq("completed"))
      .get_result(&conn)?;
    Ok(result)
  }

  pub fn _error(
    &self,
    transaction_id: String,
    reason: String,
  ) -> Result<TransactionModel, ApiError> {
    //connection
    let conn = connection()?;
    //find transaction
    let mut find_transaction = self.transaction_repo.find_by_id(transaction_id)?;
    // change status
    find_transaction.cancel(reason);
    // update registry
    let result: TransactionModel = diesel::update(transaction::table)
      .set(&find_transaction)
      .get_result(&conn)?;
    print!("{:?}", result);
    Ok(result)
  }

  pub fn new<
    P: 'static + PixKeyRepositoryInterface + Send,
    T: 'static + TransactionRepositoryInterface + Send,
  >(
    pixkey_repo: P,
    transaction_repo: T,
  ) -> TransactionUseCase {
    TransactionUseCase {
      pixkey_repo: Box::new(pixkey_repo),
      transaction_repo: Box::new(transaction_repo),
    }
  }
}
