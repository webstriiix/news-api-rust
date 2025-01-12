use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use crate::db::DBPool;
use crate::models::news::News;
use crate::schema::news::dsl::*;

pub async fn list_news(pool: web::Data<DBPool>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection.");
    let all_news = news
        .load::<News>(&mut conn)
        .expect("Error loading news.");

    HttpResponse::Ok().json(all_news)
}

