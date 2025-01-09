CREATE TABLE news_categories (
    news_id INTEGER NOT NULL REFERENCES news(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    PRIMARY KEY (news_id, category_id)
);
