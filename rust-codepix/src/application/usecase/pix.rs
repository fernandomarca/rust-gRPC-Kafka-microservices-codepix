use crate::{
  api_error::ApiError,
  domain::{
    self,
    model::{account::AccountModel, bank::BankModel, pix_key::PixKeyModel},
  },
  infrastructure::repository::pix::PixResult,
};
use domain::model::pix_key::PixKeyRepositoryInterface;

pub struct PixUseCase {
  pix_key_repo: Box<dyn PixKeyRepositoryInterface + Send>,
}

impl PixUseCase {
  pub fn new<P: 'static + PixKeyRepositoryInterface + Send>(pix_key_repo: P) -> PixUseCase {
    PixUseCase {
      pix_key_repo: Box::new(pix_key_repo),
    }
  }
  pub fn register_key(
    &self,
    kind: String,
    key: String,
    account_id: String,
  ) -> Result<PixKeyModel, ApiError> {
    // find account
    let account = self.pix_key_repo.find_account(&account_id)?;
    //register pix
    let result = self.pix_key_repo.register_key(key, kind, account.id);
    result
  }
  pub fn _find_account(&self, id: String) -> Result<AccountModel, ApiError> {
    let account = self.pix_key_repo.find_account(&id);
    account
  }

  pub fn find_key(&self, key: String) -> Result<PixResult, ApiError> {
    let pix_key = self.pix_key_repo.find_key_by_key(&key);
    pix_key
  }
  pub fn find_pix_by_id(&self, id: &String) -> Result<PixResult, ApiError> {
    let pix_key = self.pix_key_repo.find_pix_by_id(id);
    pix_key
  }

  pub fn _find_bank(&self, id: String) -> Result<BankModel, ApiError> {
    let bank = self.pix_key_repo.find_bank(id);
    bank
  }
}
