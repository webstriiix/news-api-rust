use crate::schema::news;
use crate::schema::news_categories;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "news"]
pub struct News {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[table_name = "news_categories"]
pub struct NewsCategory {
    pub news_id: i32,
    pub category_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct NewsWithCategories {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub categories: Vec<String>
}
