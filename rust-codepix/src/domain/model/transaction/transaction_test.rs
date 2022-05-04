// #[cfg(test)]
// mod transaction_test {

//   use crate::{
//     domain::model,
//     infrastructure::prisma_db::db::{AccountP, AccountPData},
//   };
//   use model::{
//     bank::Bank,
//     pix_key::PixKey,
//     transaction::{Transaction, TransactionStatus},
//   };

//   fn account_mock() -> AccountPData {
//     let bank = Bank::new("001".to_string(), String::from("Banco do Brasil")).unwrap();

//     let account = AccountP::new(bank, String::from("123456"), String::from("Jhon Doe"));

//     account.unwrap()
//   }

//   fn account_from_mock() -> AccountPData {
//     let bank = Bank::new("001".to_string(), String::from("Banco do Brasil")).unwrap();

//     let account = AccountP::new(bank, String::from("78910"), String::from("Jhon Doe"));

//     account.unwrap()
//   }

//   fn pix_key_mock() -> PixKey {
//     let account = account_mock();

//     let pix_key = PixKey::new(
//       String::from("email"),
//       account,
//       String::from("jhondoe@test.com"),
//     );

//     pix_key.unwrap()
//   }

//   #[test]
//   fn transaction_new() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     assert!(transaction.is_ok());
//   }

//   #[test]
//   fn transaction_amount_greater_than_0() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       0,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     assert!(transaction.is_err());
//   }

//   #[test]
//   fn transaction_status() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let status = transaction.unwrap().status;
//     assert!(status == TransactionStatus::TransactionPending);
//   }

//   #[test]
//   fn transaction_destination_account_cannot_be_the_same() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let pix_key_to_account_id = transaction.unwrap().pix_key_to.account_id;
//     let account_from_id = account_from.id;
//     assert!(pix_key_to_account_id != account_from_id);
//   }

//   use pretty_assertions::assert_ne;

//   #[test]
//   fn transaction_number_bank_no_equal() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let pix_key_to_account_number = transaction.unwrap().pix_key_to.account.number;
//     let account_from_number = account_from.number;
//     assert_ne!(pix_key_to_account_number, account_from_number);
//   }

//   #[test]
//   fn transaction_complete_test() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let mut result = transaction.unwrap();
//     let transaction_complete = result.complete();
//     assert!(transaction_complete.is_ok());
//     assert!(result.status == TransactionStatus::TransactionCompleted);
//   }

//   #[test]
//   fn transaction_confirm_test() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let mut result = transaction.unwrap();
//     let transaction_confirm = result.confirm();
//     assert!(transaction_confirm.is_ok());
//     assert!(result.status == TransactionStatus::TransactionConfirmed);
//   }

//   #[test]
//   fn transaction_cancel_test() {
//     let account_from = account_from_mock();
//     let pix_key_to = pix_key_mock();
//     let transaction = Transaction::new(
//       account_from,
//       100,
//       &pix_key_to,
//       String::from("transaction test"),
//     );
//     let mut result = transaction.unwrap();
//     let transaction_cancel = result.cancel(String::from("transaction test cancel"));
//     assert!(transaction_cancel.is_ok());
//     assert!(result.status == TransactionStatus::TransactionError);
//   }
// }
