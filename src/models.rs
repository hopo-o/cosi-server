use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
  pub id: u32,
  pub name: String,
  pub phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
  pub id: u32,
  pub description: String,
  pub status: String,
}
