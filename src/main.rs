use actix_web::{web, App, HttpServer};

mod models;
mod persistence;
mod routes;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let db_user = String::from("root");
    let db_password = String::from("qqq12138");
    let db_host = String::from("localhost");
    let db_port = 3306;
    let db_name = String::from("cosi");

    let builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));

    log::info!("initializing database connection");

    let pool = mysql::Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);

    log::info!(
        "starting HTTP server at http://localhost:8080"
    );

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(routes::index)
            .service(routes::user_detail)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
