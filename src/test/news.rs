#[cfg(test)]
mod tests {
    use crate::models::news::News;

    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    // Mock database
    struct MockDb {
        news_store: Arc<Mutex<HashMap<i32, News>>>,
    }

    impl MockDb {
        fn new() -> Self {
            MockDb {
                news_store: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn insert_news(&self, id: i32, title: &str, content: &str, author_id: i32) {
            let mut store = self.news_store.lock().unwrap();
            store.insert(
                id,
                News {
                    id,
                    title: title.to_string(),
                    content: content.to_string(),
                    author_id,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                },
            );
        }

        fn update_news(
            &self,
            id: i32,
            new_title: &str,
            new_content: &str,
        ) -> Result<News, &'static str> {
            let mut store = self.news_store.lock().unwrap();
            if let Some(news) = store.get_mut(&id) {
                news.title = new_title.to_string();
                news.content = new_content.to_string();
                news.updated_at = Utc::now().naive_utc();
                return Ok(news.clone());
            }
            Err("News not found")
        }

        fn delete_news(&self, id: i32) -> Result<(), &'static str> {
            let mut store = self.news_store.lock().unwrap();
            if store.remove(&id).is_some() {
                return Ok(());
            }
            Err("News not found")
        }
    }

    #[test]
    fn test_update_news_success() {
        let db = MockDb::new();
        db.insert_news(1, "Original Title", "Original Content", 1);

        let updated_news = db.update_news(1, "Updated Title", "Updated Content");
        assert!(updated_news.is_ok());

        let news = updated_news.unwrap();
        assert_eq!(news.title, "Updated Title");
        assert_eq!(news.content, "Updated Content");
    }

    #[test]
    fn test_update_news_not_found() {
        let db = MockDb::new();
        let result = db.update_news(99, "New Title", "New Content");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "News not found");
    }

    #[test]
    fn test_delete_news_success() {
        let db = MockDb::new();
        db.insert_news(1, "Test News", "Some Content", 1);

        let result = db.delete_news(1);
        assert!(result.is_ok());

        let check = db.delete_news(1);
        assert!(check.is_err()); // News should be gone now
    }

    #[test]
    fn test_delete_news_not_found() {
        let db = MockDb::new();
        let result = db.delete_news(99);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "News not found");
    }
}
