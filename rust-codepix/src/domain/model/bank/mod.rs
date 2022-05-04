mod bank_test;
use crate::infrastructure::db::schema::bank;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
  Debug, Deserialize, Serialize, Queryable, Identifiable, Clone, Associations, AsChangeset,
)]
#[table_name = "bank"]
//#[belongs_to(AccountModel, foreign_key = "accounts")]
pub struct BankModel {
  pub id: String,
  pub code: String,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
  pub accounts: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "bank"]
pub struct NewBank {
  pub id: String,
  pub code: String,
  pub name: String,
}

impl NewBank {
  pub fn new(code: String, name: String) -> NewBank {
    let bank = NewBank {
      id: Uuid::new_v4().to_string(),
      code,
      name,
    };
    bank
  }
}
