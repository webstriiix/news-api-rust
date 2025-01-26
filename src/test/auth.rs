#[cfg(test)]
mod auth_tests {
    use crate::schema::users::dsl::*;
    use crate::{db::establish_connection, routes::configure_routes};
    use actix_web::{http::StatusCode, test, web, App};
    use diesel::prelude::*;
    use dotenvy::dotenv;
    use serde_json::json;

    #[actix_web::test]
    async fn test_register() {
        dotenv().ok();
        let pool = establish_connection();
        let conn = &mut pool.get().unwrap();

        // Clean up the test user if it exists
        diesel::delete(users.filter(username.eq("test_user")))
            .execute(conn)
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(configure_routes),
        )
        .await;

        // Register a new user
        let register_req = test::TestRequest::post()
            .uri("/auth/register")
            .set_json(json!({
                "username": "test_user",
                "password": "test_password"
            }))
            .to_request();

        let register_resp = test::call_service(&app, register_req).await;
        assert_eq!(register_resp.status(), StatusCode::CREATED);

        let register_body: serde_json::Value = test::read_body_json(register_resp).await;
        assert_eq!(register_body, "User created successfully");
    }

    #[actix_web::test]
    async fn test_login_success() {
        dotenv().ok();
        let pool = establish_connection();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(configure_routes),
        )
        .await;

        // Log in with valid credentials
        let login_req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "username": "test_user",
                "password": "test_password"
            }))
            .to_request();

        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::OK);

        let login_body: serde_json::Value = test::read_body_json(login_resp).await;
        assert!(login_body.get("token").is_some());
        assert_eq!(login_body["is_admin"], false);
    }

    #[actix_web::test]
    async fn test_login_failure() {
        dotenv().ok();
        let pool = establish_connection();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .configure(configure_routes),
        )
        .await;

        // Log in with invalid credentials
        let login_req = test::TestRequest::post()
            .uri("/auth/login")
            .set_json(json!({
                "username": "test_user",
                "password": "wrong_password"
            }))
            .to_request();

        let login_resp = test::call_service(&app, login_req).await;
        assert_eq!(login_resp.status(), StatusCode::UNAUTHORIZED);

        // Read the response body as a string
        let body = test::read_body(login_resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();

        // Assert the plain text error message
        assert_eq!(body_str, "Invalid credentials");
    }
}
