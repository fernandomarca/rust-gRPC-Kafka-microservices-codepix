pub mod schema;
use crate::api_error::ApiError;
use diesel::Connection;
use diesel::{
  pg::PgConnection,
  r2d2::{ConnectionManager, ManageConnection, Pool, PooledConnection},
};
use lazy_static::lazy_static;
use log::info;
use std::env;

embed_migrations!();

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
  static ref POOL: PgPool = {
    let ambient = env::var("AMBIENT").expect("env ambient error");
    let db_url = if let true = ambient == "dev".to_string() {
      env::var("DATABASE_URL_DEV").expect("env DATABASE_URL_DEV error")
    } else {
      env::var("DATABASE_URL").expect("env DATABASE_URL error")
    };
    //
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Failed to create db pool")
  };
}

pub fn connection() -> Result<DbConnection, ApiError> {
  POOL
    .get()
    .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn init() {
  info!("Initializing DB");
  lazy_static::initialize(&POOL);
  let conn = connection().expect("Failed to get db connection");
  embedded_migrations::run(&conn).expect("Failed to run database migrations");
}

fn _create_connection_pool_pg() -> PgPool {
  let db_url = env::var("DATABASE_URL_DEV").expect("Can't get DB URL");
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::builder()
    .build(manager)
    .expect("Failed to create pool")
}

fn _create_connection_pool<T>(env: &str) -> Pool<ConnectionManager<T>>
where
  T: ManageConnection + Connection,
{
  match env == "test" {
    true => {
      let db_url = env::var("DATABASE_URL_TEST").expect("Can't get DB URL");
      let manager = ConnectionManager::<T>::new(db_url);
      let db = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
      db
    }
    false => {
      let db_url = env::var("DATABASE_URL_DEV").expect("Can't get DB URL");
      let manager = ConnectionManager::<T>::new(db_url);
      let db = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");
      db
    }
  }
}
