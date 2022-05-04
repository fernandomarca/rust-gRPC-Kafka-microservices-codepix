use crate::application::grpc::pixgrpc::pixkey::PixKeyResponse;

use super::PixKeyModel;
use chrono::NaiveDateTime;

const NANOS_PER_SECOND: i64 = 1000000;
fn chrono_to_prost_timestamp(timestamp: NaiveDateTime) -> prost_types::Timestamp {
  let nanos = timestamp.timestamp_nanos();
  let seconds = nanos / NANOS_PER_SECOND;
  let nanos = ((nanos % NANOS_PER_SECOND) * 100) as i32;
  prost_types::Timestamp { nanos, seconds }
}

fn _prost_timestamp_to_chrono(timestamp: prost_types::Timestamp) -> NaiveDateTime {
  let nanos = timestamp.seconds * NANOS_PER_SECOND;
  let nsecs = timestamp.nanos as u32;
  NaiveDateTime::from_timestamp(nanos, nsecs)
}

//conversion Model for PixKeyResponse
impl From<PixKeyModel> for PixKeyResponse {
  fn from(px: PixKeyModel) -> PixKeyResponse {
    let created_at = chrono_to_prost_timestamp(px.created_at);
    let updated_at = chrono_to_prost_timestamp(px.updated_at);
    PixKeyResponse {
      id: px.id,
      kind: px.kind,
      key: px.key,
      created_at: Some(created_at),
      updated_at: Some(updated_at),
      account_id: px.account_id,
      status: px.status,
    }
  }
}

// pub struct PixKeyCreatedResult {
//   pub id: String,
//   pub status: String,
// }

// impl From<PixKeyPData> for PixKeyCreatedResult {
//   fn from(t: PixKeyPData) -> Self {
//     Self {
//       id: t.id,
//       status: t.status,
//     }
//   }
// }
