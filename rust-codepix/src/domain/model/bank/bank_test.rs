// #[cfg(test)]
// mod bank_tests {
//   use crate::domain::model::bank::Bank;
//   use pretty_assertions::assert_eq;
//   use uuid::Uuid;
//   #[test]
//   fn create_new_bank() {
//     let bank = Bank::new("b01".to_string(), "bank01".to_string());
//     assert_eq!(bank.is_ok(), true);
//   }

//   #[test]
//   fn bank_with_valid_uuid() {
//     let bank = Bank::new("b01".to_string(), "bank01".to_string());
//     let id = bank.unwrap().base.id;
//     assert!(Uuid::parse_str(&id).is_ok());
//   }

//   #[test]
//   fn bank_code_required_and_not_empty_or_blank() {
//     let bank = Bank::new("b01".to_string(), "bank01".to_string());
//     let code = bank.unwrap().code;

//     assert_eq!(code.trim().is_empty(), false);
//   }

//   #[test]
//   fn bank_name_required_and_not_empty_or_blank() {
//     let bank = Bank::new("b01".to_string(), "bank01".to_string());
//     let name = bank.unwrap().name;

//     assert_eq!(name.trim().is_empty(), false);
//   }
// }
