use crate::schema::news;
use crate::schema::news_categories;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::models::category::CategorySummary;

#[derive(Queryable, Serialize, Deserialize, Insertable, Debug)]
#[diesel(table_name = news)]
pub struct News {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = news_categories)]
pub struct NewsCategory {
    pub news_id: i32,
    pub category_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NewsDetail {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub categories: Vec<CategorySummary>,
}

// struct for json response list_news
#[derive(Serialize, Deserialize)]
pub struct NewsSummary {
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
}
