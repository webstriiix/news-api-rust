use crate::models::news::News;
use crate::schema::categories::{self, dsl::*};
use crate::schema::news;
use crate::schema::news::dsl::*;
use crate::schema::news_categories;
use crate::utils::error_response::AppError;
use crate::utils::jwt::Claims;
use crate::{db::DBPool, models::category::Category, models::news::NewsCategory};
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
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

/*
        CREATE FUNCTION
*/

pub async fn create_news(
    pool: web::Data<DBPool>,
    news_data: web::Json<NewsWithCategories>,
) -> Result<HttpResponse, AppError> {
    let mut conn = pool
        .get()
        .map_err(|e| AppError::DatabaseError(format!("Failed to get DB connection: {}", e)))?;

    // Perform the transaction
    let new_news = conn.transaction::<_, AppError, _>(|conn| {
        // Insert the news item
        let new_news = diesel::insert_into(news::table)
            .values((
                news::title.eq(&news_data.title),
                news::content.eq(&news_data.content),
                news::author_id.eq(news_data.author_id),
                news::created_at.eq(Utc::now().naive_utc()),
                news::updated_at.eq(Utc::now().naive_utc()),
            ))
            .get_result::<News>(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to insert news: {}", e)))?;

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
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to insert categories: {}", e)))?;

        Ok(new_news)
    })?;

    // Create successful response
    let response = json!({
        "message": "News created successfully",
        "news": {
            "id": new_news.id,
            "title": new_news.title,
            "content": new_news.content,
            "author_id": new_news.author_id,
            "created_at": new_news.created_at,
            "updated_at": new_news.updated_at,
            "categories": news_data.category_ids
        }
    });

    Ok(HttpResponse::Created().json(response))
}

// create category
pub async fn create_category(
    pool: web::Data<DBPool>,
    category_data: web::Json<Category>,
) -> Result<HttpResponse, AppError> {
    // Get database connection
    // Validate input
    if let Err(errors) = category_data.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let mut conn = pool
        .get()
        .map_err(|e| AppError::DatabaseError(format!("Failed to get DB connection: {}", e)))?;

    // Construct the category item
    let new_category = Category {
        id: 0, // This will be replaced by the database
        name: category_data.name.clone(),
        description: category_data.description.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    // Perform insertion within a transaction
    let result = conn.transaction::<_, AppError, _>(|conn| {
        diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result::<Category>(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to create category: {}", e)))
    })?;

    Ok(HttpResponse::Created().json(json!({
        "message": "Category created successfully",
        "category": result
    })))
}

/*
        UPDATE FUNCTION
*/

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

// update the category in the news
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

// update news
pub async fn update_news(
    req: HttpRequest,
    path: web::Path<i32>,
    update_data: web::Json<UpdateNewsRequest>,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse, AppError> {
    let news_id = path.into_inner();

    // extract user claims from JWT
    let user_claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::UnauthorizedError("Unauthorized access".into()))?;

    // validate input if any fields are provided
    if let Err(errors) = update_data.validate() {
        return Ok(HttpResponse::BadRequest().json(errors));
    }

    let mut conn = pool
        .get()
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    // start updating
    let result = conn.transaction::<_, AppError, _>(|conn| {
        // Build update query dynamically based on provided fields
        let existing_news = news
            .find(news_id)
            .first::<News>(conn)
            .optional()
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("News not found!".into()))?;

        // check the user authority, only admin allowed
        if !user_claims.is_admin && user_claims.sub != existing_news.author_id {
            return Err(AppError::ForbiddenError("Not authorized!".into()));
        }

        // Build update query dynamically based on provided fields
        let changeset = NewsChangeset {
            title: update_data
                .news_title
                .as_deref()
                .or(Some(&existing_news.title)),
            content: update_data
                .news_content
                .as_deref()
                .or(Some(&existing_news.content)),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        // execute update
        let updated_news: News = diesel::update(news.find(news_id))
            .set(&changeset)
            .get_result(conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // update categories if provided
        if let Some(news_categories) = &update_data.category_ids {
            update_news_categories(conn, news_id, news_categories)
                .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        }

        Ok(updated_news)
    })?;

    Ok(HttpResponse::Ok().json(UpdateNewsResponse {
        message: "News updated successfully".to_string(),
        news: result,
    }))
}

/*
        DELETE FUNCTION
*/

pub async fn delete_news(
    req: HttpRequest,
    pool: web::Data<DBPool>,
    news_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    // extract user claims from JWT
    let user_claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::UnauthorizedError("Unauthorized access".into()))?;

    let mut conn = pool
        .get()
        .map_err(|e| AppError::DatabaseError(format!("Database connection error: {}", e)))?;

    // perform deletion within a transaction
    let result = conn.transaction::<_, AppError, _>(|conn| {
        // first check if news exists and user has permission
        let news_item = news::table
            .find(*news_id)
            .first::<News>(conn)
            .optional()
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("News not found".into()))?;

        // check authorization
        if !user_claims.is_admin && user_claims.sub != news_item.author_id {
            return Err(AppError::ForbiddenError(
                "Not authorized to delete this news".into(),
            ));
        }

        // delete associated categories first
        diesel::delete(news_categories::table.filter(news_categories::news_id.eq(*news_id)))
            .execute(conn)
            .map_err(|e| {
                AppError::DatabaseError(format!("Failed to delete news categories: {}", e))
            })?;

        // then delete the news
        diesel::delete(news::table.filter(news::id.eq(*news_id)))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete news: {}", e)))?;

        Ok(())
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "News deleted successfully"
    })))
}

pub async fn delete_category(
    req: HttpRequest,
    pool: web::Data<DBPool>,
    category_id: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    // extract user claims from JWT and ensure admin
    let user_claims = req
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::UnauthorizedError("Unauthorized access".into()))?;

    if !user_claims.is_admin {
        return Err(AppError::ForbiddenError(
            "Only admins can delete categories".into(),
        ));
    }

    let mut conn = pool
        .get()
        .map_err(|e| AppError::DatabaseError(format!("Database connection error: {}", e)))?;

    // perform deletion within a transaction
    let result = conn.transaction::<_, AppError, _>(|conn| {
        // check if category exists
        let exists = categories::table
            .find(*category_id)
            .count()
            .get_result::<i64>(conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if exists == 0 {
            return Err(AppError::NotFoundError("Category not found".into()));
        }

        // delete associated news_categories first
        diesel::delete(
            news_categories::table.filter(news_categories::category_id.eq(*category_id)),
        )
        .execute(conn)
        .map_err(|e| {
            AppError::DatabaseError(format!("Failed to delete category associations: {}", e))
        })?;

        // then delete the category
        diesel::delete(categories::table.filter(categories::id.eq(*category_id)))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(format!("Failed to delete category: {}", e)))?;

        Ok(())
    })?;

    Ok(HttpResponse::Ok().json(json!({
        "message": "Category deleted successfully"
    })))
}
