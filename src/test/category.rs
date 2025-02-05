#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)] // ✅ Fix: Add Clone
    struct Category {
        id: i32,
        name: String,
        description: String,
        created_at: chrono::NaiveDateTime,
        updated_at: chrono::NaiveDateTime,
    }

    struct MockDb {
        category_store: Arc<Mutex<HashMap<i32, Category>>>,
    }

    impl MockDb {
        fn new() -> Self {
            MockDb {
                category_store: Arc::new(Mutex::new(HashMap::new())),
            }
        }

        fn insert_category(&self, id: i32, name: &str, description: &str) {
            let mut store = self.category_store.lock().unwrap();
            store.insert(
                id,
                Category {
                    id,
                    name: name.to_string(),
                    description: description.to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                },
            );
        }

        fn update_category(
            &self,
            id: i32,
            new_name: &str,
            new_description: &str,
        ) -> Result<Category, &'static str> {
            let mut store = self.category_store.lock().unwrap();
            if let Some(category) = store.get_mut(&id) {
                category.name = new_name.to_string();
                category.description = new_description.to_string();
                category.updated_at = Utc::now().naive_utc();
                return Ok(category.clone()); // ✅ Now it works!
            }
            Err("Category not found")
        }

        fn delete_category(&self, id: i32) -> Result<(), &'static str> {
            let mut store = self.category_store.lock().unwrap();
            if store.remove(&id).is_some() {
                return Ok(());
            }
            Err("Category not found")
        }
    }

    #[test]
    fn test_create_category_success() {
        let db = MockDb::new();
        db.insert_category(1, "Tech", "Technology related articles");

        let store = db.category_store.lock().unwrap();
        let category = store.get(&1).expect("Category should exist");

        assert_eq!(category.name, "Tech");
        assert_eq!(category.description, "Technology related articles");
    }

    #[test]
    fn test_update_category_success() {
        let db = MockDb::new();
        db.insert_category(1, "Tech", "Technology related articles");

        let updated_category = db.update_category(1, "Science", "Science related articles");
        assert!(updated_category.is_ok());

        let category = updated_category.unwrap();
        assert_eq!(category.name, "Science");
        assert_eq!(category.description, "Science related articles");
    }

    #[test]
    fn test_update_category_not_found() {
        let db = MockDb::new();
        let result = db.update_category(99, "New Name", "New Description");

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Category not found");
    }

    #[test]
    fn test_delete_category_success() {
        let db = MockDb::new();
        db.insert_category(1, "Tech", "Technology related articles");

        let result = db.delete_category(1);
        assert!(result.is_ok());

        let check = db.delete_category(1);
        assert!(check.is_err()); // Category should be gone now
    }

    #[test]
    fn test_delete_category_not_found() {
        let db = MockDb::new();
        let result = db.delete_category(99);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Category not found");
    }
}
