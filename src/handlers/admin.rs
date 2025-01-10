use crate::db::DBPool;
use crate::models::news::News;
use crate::schema::news::dsl::*;
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub async fn create_news(pool: web::Data<DBPool>, news_data: web::Json<News>) -> HttpResponse {
    // Get a database connection from the pool
    let mut conn = pool.get().expect("Failed to get DB connection.");

    // Construct the new news item
    let new_news = News {
        id: 0, // Will auto-increment
        title: news_data.title.clone(),
        content: news_data.content.clone(),
        author_id: news_data.author_id,
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // Perform the database insertion
    match diesel::insert_into(news)
        .values(&new_news)
        .execute(&mut conn) // Pass mutable reference
    {
        Ok(_) => HttpResponse::Created().json(new_news),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving new news: {}", e)),
    }
}
