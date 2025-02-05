use actix_web::{web, App};
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use std::env;
use uuid::Uuid;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

/// Creates a new test database and returns its connection URL
pub fn create_test_database() -> String {
    dotenv().ok();
    let base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let test_db_name = format!("test_db_{}", Uuid::new_v4().to_string().replace("-", ""));

    // Connect to postgres database to create test database
    let postgres_url = base_url.replace("/news_api", "/postgres");
    let mut conn =
        PgConnection::establish(&postgres_url).expect("Failed to connect to postgres database");

    // Create the test database
    diesel::sql_query(format!("CREATE DATABASE {}", test_db_name))
        .execute(&mut conn)
        .expect("Failed to create test database");

    // Return the URL for the new test database
    base_url.replace("/news_api", &format!("/{}", test_db_name))
}

/// Creates a connection pool for testing
pub fn get_test_pool() -> (DBPool, String) {
    let database_url = create_test_database();
    let manager = ConnectionManager::<PgConnection>::new(database_url.clone());
    let pool = r2d2::Pool::builder()
        .max_size(2) // Small pool size for testing
        .build(manager)
        .expect("Failed to create test pool");

    (pool, database_url)
}

/// Cleans up the test database
pub fn cleanup_test_database(database_url: &str) {
    let postgres_url = database_url.replace(database_url.split('/').last().unwrap(), "postgres");

    let mut conn =
        PgConnection::establish(&postgres_url).expect("Failed to connect to postgres database");

    let db_name = database_url
        .split('/')
        .last()
        .expect("Invalid database URL");

    // Safety check
    if !db_name.starts_with("test_db_") {
        panic!("Attempting to delete non-test database: {}", db_name);
    }

    // Terminate existing connections
    let terminate_query = format!(
        "SELECT pg_terminate_backend(pid) 
         FROM pg_stat_activity 
         WHERE datname = '{}'",
        db_name
    );
    let _ = diesel::sql_query(&terminate_query).execute(&mut conn);

    // Drop the database
    let drop_query = format!("DROP DATABASE IF EXISTS {}", db_name);
    diesel::sql_query(&drop_query)
        .execute(&mut conn)
        .expect("Failed to drop test database");
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;
    use diesel::sql_query;

    #[test]
    async fn test_database_lifecycle() {
        // Create test database and pool
        let (pool, database_url) = get_test_pool();

        // Check if DB is reachable
        let mut conn = pool.get().expect("Failed to get connection");
        let result: Result<(), _> = sql_query("SELECT 1").execute(&mut conn).map(|_| ());
        assert!(result.is_ok(), "Database connection is not alive");
        // Clean up (important to drop connections first)
        drop(conn);
        drop(pool);
        cleanup_test_database(&database_url);

        // Verify database was deleted
        assert!(
            PgConnection::establish(&database_url).is_err(),
            "Database still exists after cleanup"
        );
    }
}
