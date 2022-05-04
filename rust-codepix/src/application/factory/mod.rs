use super::usecase::transaction::TransactionUseCase;
use crate::application::usecase::pix::PixUseCase;
use crate::infrastructure::db::connection;
use crate::infrastructure::{
  repository::pix::PixkeyRepositoryDb, repository::transaction::TransactionRepoDb,
};

pub fn transaction_usecase_factory() -> TransactionUseCase {
  let database = connection().expect("conection db error");
  let pix_repository = PixkeyRepositoryDb::new(database);
  let transaction_repository = TransactionRepoDb::new();
  let transaction_usecase = TransactionUseCase::new(pix_repository, transaction_repository);
  transaction_usecase
}

pub fn pixkey_usecase_factory() -> PixUseCase {
  let database = connection().expect("conection db error");
  let pix_repository = PixkeyRepositoryDb::new(database);
  let pixkey_usecase = PixUseCase::new(pix_repository);

  pixkey_usecase
}
