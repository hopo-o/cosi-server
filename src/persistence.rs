use actix_web::http::StatusCode;
use mysql::{params, prelude::Queryable};

use crate::models;

#[derive(Debug, derive_more::Display, derive_more::Error, derive_more::From)]
pub enum PersistenceError {
    UserNotFound,
    SqlError(mysql::Error),
    _Unknown,
}

impl actix_web::ResponseError for PersistenceError {
  fn status_code(&self) -> StatusCode {
    match self {
      PersistenceError::UserNotFound => StatusCode::NOT_FOUND,
      PersistenceError::SqlError(_) | PersistenceError::_Unknown => StatusCode::INTERNAL_SERVER_ERROR        
    }
  }
     
}



pub(crate) fn get_user_by_id(
    pool: &mysql::Pool,
    user_id: u32,
) -> Result<models::User, PersistenceError> {
    let mut conn = pool.get_conn()?;

    // get first user
    if let Some(user) = select_user_by_id(&mut conn, user_id)?.into_iter().next() {
      Ok(user)
    } else {
      Err(PersistenceError::UserNotFound)
    }
}

fn select_user_by_id(
    conn: &mut mysql::PooledConn,
    user_id: u32,
) -> mysql::error::Result<Vec<models::User>> {
    conn.exec_map(
      r"SELECT id, name, phone FROM user WHERE id = :user_id", 
      params! {user_id}, 
      |(id, name, phone)| {
      models::User { id, name, phone }
    })
}
