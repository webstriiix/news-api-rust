use crate::schema::categories;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Queryable, Serialize, Deserialize, Insertable, Validate)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
    #[validate(length(min = 1, message = "Description is required"))]
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
    #[validate(length(
        min = 3,
        max = 100,
        message = "Category name must be between 3 and 100 characters!"
    ))]
    pub name: String,

    #[validate(length(
        min = 3,
        max = 255,
        message = "Category description must be between 3 and 255 characters!"
    ))]
    pub description: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = categories)]
pub struct CategoryChangeset {
    pub name: Option<String>,        // Use Option<String> for optional updates
    pub description: Option<String>, // Use Option<String> for optional updates
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct UpdateCategoryResponse {
    pub message: String,
    pub category: Category,
}
