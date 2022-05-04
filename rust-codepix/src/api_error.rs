use diesel::result::Error as DieselError;
use serde::Deserialize;
use std::fmt;
use tonic::Status;

#[derive(Debug, Deserialize)]
pub struct ApiError {
  pub status_code: u16,
  pub message: String,
}

pub struct ApiErrorGrpc {}

impl ApiError {
  pub fn new(status_code: u16, message: String) -> ApiError {
    ApiError {
      status_code,
      message,
    }
  }
}

impl ApiErrorGrpc {
  pub fn new(error: ApiError) -> Status {
    error.into()
  }
}

impl fmt::Display for ApiError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_str(self.message.as_str())
  }
}

impl From<DieselError> for ApiError {
  fn from(error: DieselError) -> ApiError {
    match error {
      DieselError::DatabaseError(_, err) => ApiError::new(409, err.message().to_string()),
      DieselError::NotFound => ApiError::new(404, "Record not found".to_string()),
      err => ApiError::new(500, format!("Internal server error: {}", err)),
    }
  }
}

impl From<ApiError> for Status {
  fn from(error: ApiError) -> Status {
    match error.status_code {
      409 => {
        let status = Status::permission_denied(format!(
          "permission denied: code: {}, message: {}",
          error.status_code, error.message
        ));
        status
      }
      404 => {
        let status = Status::not_found(format!(
          "Record not found: code: {}, message: {}",
          error.status_code, error.message
        ));
        status
      }
      _ => {
        let status = Status::unknown(format!(
          "Internal server error: code: {}, message: {}",
          error.status_code, error.message
        ));
        status
      }
    }
  }
}

// impl ResponseError for ApiError {
//   fn error_response(&self) -> HttpResponse {
//     let status_code = match StatusCode::from_u16(self.status_code) {
//       Ok(status_code) => status_code,
//       Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//     };

//     let message = match status_code.as_u16() < 500 {
//       true => self.message.clone(),
//       false => {
//         error!("{}", self.message);
//         "Internal server error".to_string()
//       }
//     };

//     HttpResponse::build(status_code).json(json!({ "message": message }))
//   }
// }
