use actix_web::http::StatusCode;
use mysql::{params, prelude::Queryable};

use crate::models;

#[derive(
  Debug,
  derive_more::Display,
  derive_more::Error,
  derive_more::From,
)]

pub enum PersistenceError {
  SqlError(mysql::Error),
  UserNotFound,
  TodoNotFound,
  _Unknown,
}

impl actix_web::ResponseError for PersistenceError {
  fn status_code(&self) -> StatusCode {
    match self {
      PersistenceError::UserNotFound
      | PersistenceError::TodoNotFound => {
        StatusCode::NOT_FOUND
      }
      PersistenceError::SqlError(_)
      | PersistenceError::_Unknown => {
        StatusCode::INTERNAL_SERVER_ERROR
      }
    }
  }
}

pub(crate) fn get_user_by_id(
  pool: &mysql::Pool,
  user_id: u32,
) -> Result<models::User, PersistenceError> {
  let mut conn = pool.get_conn()?;

  // get first user
  if let Some(user) = select_user_by_id(&mut conn, user_id)?
    .into_iter()
    .next()
  {
    Ok(user)
  } else {
    Err(PersistenceError::UserNotFound)
  }
}

pub(crate) fn get_todo_list_by_user_id(
  pool: &mysql::Pool,
  user_id: u32,
) -> Result<Vec<models::Todo>, PersistenceError> {
  let mut conn = pool.get_conn()?;

  Ok(select_todo_list_by_user_id(&mut conn, user_id)?)
}

pub(crate) fn get_todo_detail_by_id(
  pool: &mysql::Pool,
  id: u32,
) -> Result<models::Todo, PersistenceError> {
  let mut conn = pool.get_conn()?;

  Ok(
    select_todo_list_by_id(&mut conn, id)?
      .into_iter()
      .next()
      .ok_or(PersistenceError::TodoNotFound)?,
  )
}

fn select_user_by_id(
  conn: &mut mysql::PooledConn,
  user_id: u32,
) -> mysql::error::Result<Vec<models::User>> {
  conn.exec_map(
    r"SELECT id, name, phone FROM user WHERE id = :user_id",
    params! {user_id},
    |(id, name, phone)| models::User { id, name, phone },
  )
}

fn select_todo_list_by_user_id(
  conn: &mut mysql::PooledConn,
  user_id: u32,
) -> mysql::error::Result<Vec<models::Todo>> {
  conn.exec_map(
    r"SELECT id, status, description FROM todo WHERE user_id = :user_id",
    params! { user_id },
    |(id, status, description)| models::Todo {
      id,
      status,
      description,
    },
  )
}

fn select_todo_list_by_id(
  conn: &mut mysql::PooledConn,
  id: u32,
) -> mysql::error::Result<Vec<models::Todo>> {
  conn.exec_map(
    r"SELECT id, status, description FROM todo WHERE id = :id",
    params! { id },
    |(id, status, description)| models::Todo {
      id,
      status,
      description,
    },
  )
}
