use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Serialize;

use crate::db::DBPool;
use crate::schema::news::dsl::*;

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
