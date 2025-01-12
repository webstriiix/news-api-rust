use actix_web::{App, HttpServer, web};
use dotenvy::dotenv;

mod db;
mod handlers;
mod models;
mod schema;
mod routes;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

