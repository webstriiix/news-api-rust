use crate::models::news::News;
use crate::schema::categories::{self, dsl::*};
use crate::schema::news;
use crate::schema::news::dsl::*;
use crate::schema::news_categories;
use crate::{db::DBPool, models::category::Category, models::news::NewsCategory};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

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

// struct for news update object
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNewsRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "title must be between 1 and 100 characters"
    ))]
    pub news_title: Option<String>,

    #[validate(length(min = 1, message = "content cannot empty"))]
    pub news_content: Option<String>,
    pub category_ids: Option<Vec<i32>>,
}

#[derive(AsChangeset)]
#[table_name = "news"]
struct NewsChangeset<'a> {
    title: Option<&'a str>,
    content: Option<&'a str>,
    updated_at: chrono::NaiveDateTime,
}

// update response object
#[derive(Debug, Serialize)]
pub struct UpdateNewsResponse {
    pub message: String,
    pub news: News,
}

// update news
pub async fn update_news(
    path: web::Path<i32>,
    update_data: web::Json<UpdateNewsRequest>,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let news_id = path.into_inner();

    // validate input if any fields is provided
    if let Err(errors) = update_data.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let conn = &mut pool.get().map_err(|e| {
        actix_web::error::ErrorInternalServerError(format!("Database error: {}", e))
    })?;

    // start updating
    let result = conn.transaction(|conn| {
        // Build update query dynamically based on provided fields
        let news_exist = news.find(news_id).first::<News>(conn).optional()?;

        if let Some(existing_news) = news_exist {
            // Build update query dynamically based on provided fields
            let changeset = NewsChangeset {
                title: update_data.news_title.as_deref(),
                content: update_data.news_content.as_deref(),
                updated_at: chrono::Utc::now().naive_utc(),
            };

            // execute update
            let updated_news: News = diesel::update(news.find(news_id))
                .set(&changeset)
                .get_result(conn)?;

            // update categories if provided
            if let Some(news_categories) = &update_data.category_ids {
                update_news_categories(conn, news_id, news_categories)?;
            }

            Ok(updated_news)
        } else {
            Err(diesel::result::Error::NotFound)
        }
    });

    match result {
        Ok(updated_news) => Ok(HttpResponse::Ok().json(UpdateNewsResponse {
            message: "News update successfully".to_string(),
            news: updated_news,
        })),
        Err(diesel::result::Error::NotFound) => Ok(HttpResponse::NotFound().json("News not found")),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(e)),
    }
}

// update the news category
fn update_news_categories(
    conn: &mut PgConnection,
    news_ids: i32,
    categoriy_ids: &[i32],
) -> QueryResult<()> {
    use crate::schema::news_categories::dsl::*;

    // delete current categories
    diesel::delete(news_categories.filter(news_id.eq(news_id))).execute(conn)?;

    // get categories entries
    let new_entries: Vec<NewsCategory> = categoriy_ids
        .iter()
        .map(|&cat_id| NewsCategory {
            news_id: news_ids,
            category_id: cat_id,
        })
        .collect();

    // insert categories
    diesel::insert_into(news_categories)
        .values(&new_entries)
        .execute(conn)?;

    Ok(())
}

// delete news
pub async fn delete_news(pool: web::Data<DBPool>, news_id: web::Path<i32>) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to connect to database!")
        }
    };

    // deleting news
    match diesel::delete(news::table.filter(news::id.eq(*news_id))).execute(&mut conn) {
        Ok(affected_rows) => {
            if affected_rows == 0 {
                HttpResponse::NotFound().body("News not found!")
            } else {
                HttpResponse::Ok().body("News deleted successfully.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete news!"),
    }
}

// delete category
pub async fn delete_category(pool: web::Data<DBPool>, category_id: web::Path<i32>) -> HttpResponse {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to connect to database!")
        }
    };

    // deleting category
    match diesel::delete(categories::table.filter(categories::id.eq(*category_id))).execute(&mut conn) {
        Ok(affected_rows) => {
            if affected_rows == 0 {
                HttpResponse::NotFound().body("Category not found!")
            } else {
                HttpResponse::Ok().body("Category deleted successfully.")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete category!"),
    }
}
