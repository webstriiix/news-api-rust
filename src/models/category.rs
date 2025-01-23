use crate::schema::categories;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CategorySummary {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 3, max = 100, message = "Category name must be between 3 and 100 characters!"))]
    pub name: String,

    #[validate(length(min = 3, max = 255, message = "Category description must be between 3 and 255 characters!"))]
    pub description: String,
}
