use actix_web::{web, HttpResponse};
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::QueryDsl;
use serde::Serialize;

use crate::db::DBPool;
use crate::models::category::CategorySummary;
use crate::models::news::{News, NewsDetail};
use crate::schema::news::dsl::*;
use crate::schema::{categories, news_categories};

// struct for json response list_news
#[derive(Serialize)]
struct NewsSummary {
    title: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn list_news(pool: web::Data<DBPool>) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection."),
    };

    // get all news but only title and date only
    let all_news = news
        .select((title, created_at))
        .load::<(String, chrono::NaiveDateTime)>(&mut conn);

    match all_news {
        Ok(news_list) => {
            // collect all data as json response
            let response: Vec<NewsSummary> = news_list
                .into_iter()
                .map(|(news_title, news_created_at)| NewsSummary {
                    title: news_title,
                    created_at: news_created_at,
                })
                .collect();

            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            eprintln!("Error fetching news: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to fetch news.")
        }
    }
}

// get news details
pub async fn get_news_detail(pool: web::Data<DBPool>, news_id: web::Path<i32>) -> HttpResponse {
    let mut conn = pool.get().expect("Failed to get DB connection.");

    // use dsl to avoid ambiguity
    use crate::schema::news;

    // find the news item by ID
    let news_item = match news::table
        .filter(news::id.eq(*news_id))
        .first::<News>(&mut conn)
    {
        Ok(found_news) => found_news,
        Err(diesel::result::Error::NotFound) => {
            return HttpResponse::NotFound().body("News not found!")
        }
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch news!"),
    };

    // fetch associated categories
    let category_list = match news_categories::table
        .inner_join(categories::table.on(news_categories::category_id.eq(categories::id)))
        .filter(news_categories::news_id.eq(news_item.id))
        .select((categories::id, categories::name))
        .load::<(i32, String)>(&mut conn)
    {
        Ok(categories) => categories
            .into_iter()
            .map(|(category_id, category_name)| CategorySummary {
                id: category_id,
                name: category_name,
            })
            .collect::<Vec<CategorySummary>>(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch categories!"),
    };

    // create response
    let response = NewsDetail {
        id: news_item.id,
        title: news_item.title,
        content: news_item.content,
        author_id: news_item.author_id,
        created_at: news_item.created_at,
        updated_at: news_item.updated_at,
        categories: category_list,
    };

    HttpResponse::Ok().json(response)
}
