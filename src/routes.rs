use actix_web::{get, web, HttpResponse, Responder};

use crate::persistence;

#[get("/")]
pub(crate) async fn index() -> impl Responder {
  HttpResponse::Ok().body("I love cosi!")
}

#[get("/user/{id}")]
pub(crate) async fn user_detail(
  path: web::Path<u32>,
  data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
  let user_id = path.into_inner();

  let user = web::block(move || {
    persistence::get_user_by_id(&data, user_id)
  })
  .await??;

  Ok(web::Json(user))
}

#[derive(serde::Deserialize)]
struct TodoParam {
  user_id: u32,
}

#[get("/todo")]
pub(crate) async fn todo_list(
  query: web::Query<TodoParam>,
  data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
  let user_id = query.user_id;

  let todo_list = web::block(move || {
    persistence::get_todo_list_by_user_id(&data, user_id)
  })
  .await??;

  Ok(web::Json(todo_list))
}

#[get("/todo/{id}")]
pub(crate) async fn todo_detail(
  path: web::Path<u32>,
  data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
  let todo_id = path.into_inner();

  let todo = web::block(move || {
    persistence::get_todo_detail_by_id(&data, todo_id)
  })
  .await??;

  Ok(web::Json(todo))
}
