use crate::models::news::News;
use crate::schema::categories::{self, dsl::*};
use crate::schema::news::dsl::*;
use crate::schema::news_categories::dsl::news_categories;
use crate::{db::DBPool, models::category::Category, models::news::NewsCategory};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewsWithCategories {
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub category_ids: Vec<i32>,
}

pub async fn create_news(
    pool: web::Data<DBPool>,
    news_data: web::Json<NewsWithCategories>,
) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection.");

    // Start a transaction
    let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insert into `news`
        let new_news = News {
            id: 0, // Will auto-increment
            title: news_data.title.clone(),
            content: news_data.content.clone(),
            author_id: news_data.author_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        let news_item = diesel::insert_into(news)
            .values(&new_news)
            .get_result::<News>(conn)?; // Use `conn` passed to the closure

        // Insert into `news_categories`
        let category_entries: Vec<NewsCategory> = news_data
            .category_ids
            .iter()
            .map(|&category_id| NewsCategory {
                news_id: news_item.id,
                category_id,
            })
            .collect();

        diesel::insert_into(news_categories)
            .values(&category_entries)
            .execute(conn)?; // Use `conn` passed to the closure

        Ok(news_item)
    });

    match transaction_result {
        Ok(news_item) => HttpResponse::Created().json(news_item),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error creating news: {}", e)),
    }
}

pub async fn create_category(
    pool: web::Data<DBPool>,
    category_data: web::Json<Category>,
) -> HttpResponse {
    // Get database connection
    let mut conn = pool.get().expect("Failed to get DB connection.");

    // Construct the category item
    let new_category = Category {
        id: 0,
        name: category_data.name.clone(),
        description: category_data.description.clone(),
        created_at: chrono::Utc::now().naive_utc(),
        updated_at: chrono::Utc::now().naive_utc(),
    };

    // perform diesel insertion
    match diesel::insert_into(categories)
        .values(&new_category)
        .execute(&mut conn)
    {
        Ok(_) => HttpResponse::Created().json(new_category),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error saving new category: {}", e))
        }
    }
}
