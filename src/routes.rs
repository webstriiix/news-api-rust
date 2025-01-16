use actix_web::web;

use crate::middleware::{admin::AdminMiddleware, auth::AuthMiddleWare};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Admin routes
    cfg.service(
        web::scope("/admin")
            .wrap(AuthMiddleWare) // First check if user is authenticated
            .wrap(AdminMiddleware) // Then check if user is admin
            .route(
                "/create-news",
                web::post().to(crate::handlers::admin::create_news),
            )
            .route(
                "/list-news",
                web::get().to(crate::handlers::news::list_news),
            )
            .route("create-category", web::post().to(crate::handlers::admin::create_category))
            .route("/register", web::post().to(crate::handlers::auth::register))
            .route("/login", web::post().to(crate::handlers::auth::login)),
    );

    // User routes
    cfg.service(web::scope("/user").route(
        "/list-news",
        web::get().to(crate::handlers::news::list_news),
    ));
}
