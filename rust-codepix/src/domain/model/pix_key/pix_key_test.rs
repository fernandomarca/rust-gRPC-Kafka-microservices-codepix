// #[allow(unused_variables)]
// #[cfg(test)]
// mod pix_key_test {
//   use std::error::Error;
//   use uuid::Uuid;
//   use validator::ValidationErrors;

//   use crate::{
//     domain::model::{account::Account, bank::Bank, pix_key::PixKey},
//     infrastructure::prisma_db::db::{AccountP, AccountPData},
//   };

//   fn factory_bank() -> Result<Bank<'static>, ValidationErrors> {
//     let bank = Bank::new("12345678".to_string(), "Bank of Rust".to_string());
//     bank
//   }

//   fn account_factory() -> Result<AccountPData, ValidationErrors> {
//     let bank = factory_bank().unwrap();
//     let account = Account::new(bank, "3838-12806-8".to_string(), "Fernando".to_string());
//     account
//   }

//   fn pix_key_factory() -> Result<PixKey, Box<dyn Error>> {
//     let account = account_factory().unwrap();
//     let pix_key = PixKey::new("email".to_string(), account, "3838-12806-8".to_string());
//     pix_key
//   }

//   #[test]
//   fn test_pix_key_new() {
//     let pix_key = pix_key_factory();
//     assert_eq!(pix_key.is_ok(), true);
//   }
//   #[test]
//   fn pix_key_with_valid_uuid() {
//     let pix_key = pix_key_factory().unwrap();
//     assert!(Uuid::parse_str(&pix_key.base.id).is_ok());
//   }

//   #[test]
//   fn required_inputs_not_empty_or_blank() {
//     let pix_key = pix_key_factory().unwrap();
//     let PixKey {
//       base,
//       kind,
//       key,
//       account_id,
//       account,
//       status,
//     } = pix_key;

//     assert_eq!(kind.trim().is_empty(), false);
//     assert_eq!(key.trim().is_empty(), false);
//     assert_eq!(account_id.trim().is_empty(), false);
//   }
// }
