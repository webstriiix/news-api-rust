use crate::models::news::News;
use crate::schema::categories::{self, dsl::*};
use crate::schema::news;
use crate::schema::news::dsl::*;
use crate::schema::news_categories;
use crate::{db::DBPool, models::category::Category, models::news::NewsCategory};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;
use serde_json::json;

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
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection."),
    };

    // Perform the transaction
    let transaction_result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
        // Insert the news item
        let new_news = diesel::insert_into(news::table)
            .values((
                news::title.eq(&news_data.title),
                news::content.eq(&news_data.content),
                news::author_id.eq(news_data.author_id),
                news::created_at.eq(chrono::Utc::now().naive_utc()),
                news::updated_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .get_result::<News>(conn)?;

        // Prepare category associations
        let category_entries: Vec<NewsCategory> = news_data
            .category_ids
            .iter()
            .map(|&category_id| NewsCategory {
                news_id: new_news.id,
                category_id,
            })
            .collect();

        // Insert into `news_categories`
        diesel::insert_into(news_categories::table)
            .values(&category_entries)
            .execute(conn)?;

        Ok(new_news)
    });

    // Return the response based on the transaction result
    match transaction_result {
        Ok(news_item) => {
            // Optionally include category information in the response
            let response = json!({
                "id": news_item.id,
                "title": news_item.title,
                "content": news_item.content,
                "author_id": news_item.author_id,
                "created_at": news_item.created_at,
                "updated_at": news_item.updated_at,
                "categories": news_data.category_ids, // Return the categories for clarity
            });
            HttpResponse::Created().json(response)
        }
        Err(e) => {
            eprintln!("Error creating news: {:?}", e); // Log the error for debugging
            HttpResponse::InternalServerError().body("Failed to create news.")
        }
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
