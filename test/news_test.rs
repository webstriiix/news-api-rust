use actix_web::{test, App};
use serde_json::json;
use crate::setup::setup_test_db;

#[actix_rt::test]
async fn test_create_news() {
    let pool = setup_test_db();

    let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes)
        ).await;

    let request = test::TestRequest::post()
        .uri("/news")
        .set_json(json!({
            "title": "Test News",
            "content": "This is a test news content",
            "author_id": 1,
            "category_ids": [1, 2]
        }))
        .to_request();

    let response = test::call_service(&mut app, request).await;
    assert_eq!(response.status(), 201);
}

#[actix_rt::test]
async fn test_get_news() {
    let pool = setup_test_db();

    let mut app = test::init_service(
            App::news()
                .app_data(web::Data::new(pool.clone()))
                .configure(routes)
        ).await;
    
    let request = test::TestRequest::get().uri("/news").to_request();
    let response = test::call_service(&mut app, request).await;

    assert_eq!(response.status(), 200);
}
