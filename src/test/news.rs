#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::category::Category;
    use crate::models::news::{News, NewsCategory, NewsDetail, NewsSummary};
    use crate::schema::{categories, news, news_categories};
    use crate::test::test_utils::{cleanup_test_database, get_test_pool, init_test_app, DBPool};
    use actix_web::test;
    use chrono::Utc;
    use diesel::prelude::*;

    async fn setup_test_data(pool: &DBPool) -> (News, Category) {
        let mut conn = pool.get().expect("Failed to get DB connection");

        // Create a test category
        let test_category = Category {
            id: 0,
            name: "Test Category".to_string(),
            description: Some("Test Description".to_string()),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        let category = diesel::insert_into(categories::table)
            .values(&test_category)
            .get_result::<Category>(&mut conn)
            .expect("Failed to create test category");

        // Create a test news article
        let test_news = News {
            id: 0,
            title: "Test News".to_string(),
            content: "Test Content".to_string(),
            author_id: 1,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        let news_item = diesel::insert_into(news::table)
            .values(&test_news)
            .get_result::<News>(&mut conn)
            .expect("Failed to create test news");

        // Create news-category association
        let news_category = NewsCategory {
            news_id: news_item.id,
            category_id: category.id,
        };

        diesel::insert_into(news_categories::table)
            .values(&news_category)
            .execute(&mut conn)
            .expect("Failed to create news-category association");

        (news_item, category)
    }

    #[actix_rt::test]
    async fn test_list_news() {
        // Setup
        let (pool, database_url) = get_test_pool();
        let app = init_test_app(pool.clone()).await;

        // Create test data
        let (news_item, _) = setup_test_data(&pool).await;

        // Make request
        let req = test::TestRequest::get().uri("/news").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert response
        assert_eq!(resp.status(), 200);

        let body: Vec<NewsSummary> = test::read_body_json(resp).await;
        assert!(!body.is_empty());
        assert_eq!(body[0].title, news_item.title);

        // Cleanup
        cleanup_test_database(&database_url);
    }

    #[actix_rt::test]
    async fn test_get_news_detail() {
        // Setup
        let (pool, database_url) = get_test_pool();
        let app = init_test_app(pool.clone()).await;

        // Create test data
        let (news_item, category) = setup_test_data(&pool).await;

        // Make request
        let req = test::TestRequest::get()
            .uri(&format!("/news/{}", news_item.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        // Assert response
        assert_eq!(resp.status(), 200);

        let body: NewsDetail = test::read_body_json(resp).await;
        assert_eq!(body.id, news_item.id);
        assert_eq!(body.title, news_item.title);
        assert_eq!(body.content, news_item.content);
        assert_eq!(body.categories.len(), 1);
        assert_eq!(body.categories[0].id, category.id);
        assert_eq!(body.categories[0].name, category.name);

        // Cleanup
        cleanup_test_database(&database_url);
    }

    #[actix_rt::test]
    async fn test_get_news_detail_not_found() {
        // Setup
        let (pool, database_url) = get_test_pool();
        let app = init_test_app(pool.clone()).await;

        // Make request with non-existent ID
        let req = test::TestRequest::get().uri("/news/999").to_request();
        let resp = test::call_service(&app, req).await;

        // Assert response
        assert_eq!(resp.status(), 404);

        // Cleanup
        cleanup_test_database(&database_url);
    }
}
