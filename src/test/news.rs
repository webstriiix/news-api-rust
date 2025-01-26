// #[cfg(test)]
// mod tests {
//     use crate::schema::users::dsl::*;
//     use crate::{db::establish_connection, models::user::User, routes::configure_routes};
//     use actix_web::{http::StatusCode, test, web, App};
//     use diesel::prelude::*;
//     use dotenvy::dotenv;
//     use serde_json::json;

//     #[actix_web::test]
//     async fn test_login_and_create_news() {
//         dotenv().ok();
//         let pool = establish_connection();
//         let conn = &mut pool.get().unwrap();

//         // Ensure the test user is an admin
//         diesel::update(users.filter(username.eq("admin_user")))
//             .set(is_admin.eq(true))
//             .execute(conn)
//             .unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(pool.clone()))
//                 .configure(configure_routes),
//         )
//         .await;

//         // Step 1: Login to get a JWT token
//         let login_req = test::TestRequest::post()
//             .uri("/auth/login")
//             .set_json(json!({
//                 "username": "admin_user",
//                 "password": "admin_password"
//             }))
//             .to_request();

//         let login_resp = test::call_service(&app, login_req).await;
//         assert_eq!(login_resp.status(), StatusCode::OK);

//         let login_body: serde_json::Value = test::read_body_json(login_resp).await;
//         let token = login_body["token"].as_str().unwrap();

//         // Step 2: Use the token to create a news item
//         let create_news_req = test::TestRequest::post()
//             .uri("/admin/create-news")
//             .set_json(json!({
//                 "title": "Test News",
//                 "content": "This is a test news article.",
//                 "author_id": 1,
//                 "category_ids": [1, 2]
//             }))
//             .insert_header(("Authorization", format!("Bearer {}", token))) // Add the JWT token
//             .to_request();

//         let create_news_resp = test::call_service(&app, create_news_req).await;
//         assert_eq!(create_news_resp.status(), StatusCode::CREATED);

//         let create_news_body: serde_json::Value = test::read_body_json(create_news_resp).await;
//         assert_eq!(create_news_body["message"], "News created successfully");
//         assert_eq!(create_news_body["news"]["title"], "Test News");
//     }

//     #[actix_web::test]
//     async fn test_login_and_update_news() {
//         dotenv().ok();
//         let pool = establish_connection();
//         let conn = &mut pool.get().unwrap();

//         // Ensure the test user is an admin
//         diesel::update(users.filter(username.eq("admin_user")))
//             .set(is_admin.eq(true))
//             .execute(conn)
//             .unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(pool.clone()))
//                 .configure(configure_routes),
//         )
//         .await;

//         // Step 1: Login to get a JWT token
//         let login_req = test::TestRequest::post()
//             .uri("/auth/login")
//             .set_json(json!({
//                 "username": "admin_user", // Replace with a valid username
//                 "password": "admin_password" // Replace with a valid password
//             }))
//             .to_request();

//         let login_resp = test::call_service(&app, login_req).await;
//         assert_eq!(login_resp.status(), StatusCode::OK);

//         let login_body: serde_json::Value = test::read_body_json(login_resp).await;
//         let token = login_body["token"].as_str().unwrap();

//         // Step 2: Use the token to update a news item
//         let update_news_req = test::TestRequest::put()
//             .uri("/admin/news-update/1") // Replace with a valid news ID
//             .set_json(json!({
//                 "news_title": "Updated Title",
//                 "news_content": "Updated content.",
//                 "category_ids": [1, 2]
//             }))
//             .insert_header(("Authorization", format!("Bearer {}", token))) // Add the JWT token
//             .to_request();

//         let update_news_resp = test::call_service(&app, update_news_req).await;
//         assert_eq!(update_news_resp.status(), StatusCode::OK);

//         let update_news_body: serde_json::Value = test::read_body_json(update_news_resp).await;
//         assert_eq!(update_news_body["message"], "News updated successfully");
//         assert_eq!(update_news_body["news"]["title"], "Updated Title");
//     }

//     #[actix_web::test]
//     async fn test_login_and_delete_news() {
//         dotenv().ok();
//         let pool = establish_connection();
//         let conn = &mut pool.get().unwrap();

//         // Ensure the test user is an admin
//         diesel::update(users.filter(username.eq("admin_user")))
//             .set(is_admin.eq(true))
//             .execute(conn)
//             .unwrap();

//         let app = test::init_service(
//             App::new()
//                 .app_data(web::Data::new(pool.clone()))
//                 .configure(configure_routes),
//         )
//         .await;

//         // Step 1: Login to get a JWT token
//         let login_req = test::TestRequest::post()
//             .uri("/auth/login")
//             .set_json(json!({
//                 "username": "admin_user", // Replace with a valid username
//                 "password": "admin_password" // Replace with a valid password
//             }))
//             .to_request();

//         let login_resp = test::call_service(&app, login_req).await;
//         assert_eq!(login_resp.status(), StatusCode::OK);

//         let login_body: serde_json::Value = test::read_body_json(login_resp).await;
//         let token = login_body["token"].as_str().unwrap();

//         // Step 2: Use the token to delete a news item
//         let delete_news_req = test::TestRequest::delete()
//             .uri("/admin/delete-news/1") // Replace with a valid news ID
//             .insert_header(("Authorization", format!("Bearer {}", token))) // Add the JWT token
//             .to_request();

//         let delete_news_resp = test::call_service(&app, delete_news_req).await;
//         assert_eq!(delete_news_resp.status(), StatusCode::OK);

//         let delete_news_body: serde_json::Value = test::read_body_json(delete_news_resp).await;
//         assert_eq!(delete_news_body["message"], "News deleted successfully");
//     }
// }
