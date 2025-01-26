use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;

mod db;
mod handlers;
mod middleware;
mod models;
mod routes;
mod schema;
mod test;
mod utils;

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
