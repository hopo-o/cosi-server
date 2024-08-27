use actix_web::{get, web, HttpResponse, Responder};

use crate::persistence;

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    HttpResponse::Ok().body("I love cosi!")
}

#[get("/users/{id}")]
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
