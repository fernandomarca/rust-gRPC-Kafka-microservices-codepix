use crate::infrastructure::db::schema::{account, bank, pixkey};
use crate::infrastructure::db::DbConnection;
use crate::{
  api_error::ApiError,
  domain::model::{
    account::{AccountModel, NewAccount},
    bank::{BankModel, NewBank},
    pix_key::{NewPix, PixKeyModel, PixKeyRepositoryInterface},
  },
};
use diesel::prelude::*;
use log::info;
pub struct PixkeyRepositoryDb {
  database: DbConnection,
}

#[derive(Debug)]
pub struct PixResult {
  pub pix: PixKeyModel,
  pub account: AccountModel,
  pub bank: BankModel,
}

impl PixkeyRepositoryDb {
  pub fn new(database: DbConnection) -> PixkeyRepositoryDb {
    PixkeyRepositoryDb { database }
  }
}

impl PixKeyRepositoryInterface for PixkeyRepositoryDb {
  fn register_key(
    &self,
    key: String,
    kind: String,
    account_id: String,
  ) -> Result<PixKeyModel, ApiError> {
    //conection Db
    let conn = &self.database;
    //register Pixkey
    let new_pix = NewPix::new(key, kind, account_id.clone());
    let pix: PixKeyModel = diesel::insert_into(pixkey::table)
      .values(&new_pix)
      .get_result(conn)?;
    //update account with pixkey
    let account: AccountModel = account::table
      .filter(account::id.eq(&account_id))
      .get_result(conn)?;
    let mut pix_keys = account.pix_keys.clone().unwrap_or_default();
    pix_keys.push(pix.id.clone());
    diesel::update(&account)
      .set(account::pix_keys.eq(pix_keys))
      .execute(conn)?;
    Ok(pix)
  }

  fn find_key_by_key(&self, key: &String) -> Result<PixResult, ApiError> {
    //conection Db
    let conn = &self.database;
    let pix: PixKeyModel = pixkey::table.filter(pixkey::key.eq(key)).first(conn)?;
    let account = self.find_account(&pix.account_id)?;
    let bank = self.find_bank(account.bank_id.clone())?;

    let result = PixResult { pix, account, bank };
    info!("{:?}", result);
    //
    Ok(result)
  }

  fn add_bank(&self, bank: NewBank) -> Result<(), ApiError> {
    //conection Db
    let conn = &self.database;
    let bank = diesel::insert_into(bank::table)
      .values(&bank)
      .execute(conn)?;
    print!("{:?}", bank);
    Ok(())
  }

  fn add_account(&self, account: NewAccount) -> Result<(), ApiError> {
    //conection Db
    let conn = &self.database;
    //find bank
    let acc_id = &account.bank_id;
    let bank_find: BankModel = bank::table.filter(bank::id.eq(acc_id)).first(conn)?;
    //insert account and update bank
    match Some(bank_find) {
      Some(bank) => {
        //insert account
        let account: AccountModel = diesel::insert_into(account::table)
          .values(&account)
          .get_result(conn)?;
        print!("{:?}", account);
        //update bank
        let mut vec_accounts = bank.accounts.clone().unwrap_or_default();
        vec_accounts.push(account.id);
        // first form to update
        // let update_bank = BankModel {
        //   accounts: Some(vec_accounts),
        //   ..bank
        // };
        // let r: BankModel = diesel::update(bank::table)
        //   .filter(bank::id.eq(acc_id))
        //   .set(update_bank)
        //   .get_result(conn)?;
        //print!("{:?}", r);
        //trendy form to update
        // let target = bank::table.filter(bank::id.eq(acc_id));
        // diesel::update(target).set(bank::accounts.eq(vec_accounts));
        //Four form to update for reference
        let result: BankModel = diesel::update(&bank)
          .set(bank::accounts.eq(vec_accounts))
          .get_result(conn)?;
        print!("{:?}", result);
      }
      None => (),
    };
    Ok(())
  }

  fn find_account(&self, id: &String) -> Result<AccountModel, ApiError> {
    //conection Db
    let conn = &self.database;
    let account: AccountModel = account::table.filter(account::id.ilike(id)).first(conn)?;
    Ok(account)
  }

  fn find_pix_by_id(&self, id: &String) -> Result<PixResult, ApiError> {
    //conection Db
    let conn = &self.database;
    let pix: PixKeyModel = pixkey::table.filter(pixkey::id.eq(id)).first(conn)?;
    let account = self.find_account(&pix.account_id)?;
    let bank = self.find_bank(account.bank_id.clone())?;

    let result = PixResult { pix, account, bank };
    info!("{:?}", result);
    //
    Ok(result)
  }

  fn find_bank(&self, id: String) -> Result<BankModel, ApiError> {
    //conection Db
    let conn = &self.database;
    let bank = bank::table.filter(bank::id.eq(id)).first(conn)?;
    Ok(bank)
  }
}
