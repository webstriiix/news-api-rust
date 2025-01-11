use actix_web::web;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    // Admin routes
    cfg.service(
        web::scope("/admin")
            .route("/create-news", web::post().to(crate::handlers::admin::create_news))
            .route("/list-news", web::get().to(crate::handlers::news::list_news))
            .route("/login", web::post().to(crate::handlers::auth::login)),
    );

    // User routes
    cfg.service(
        web::scope("/user")
            .route("/list-news", web::get().to(crate::handlers::news::list_news)),
    );
}

