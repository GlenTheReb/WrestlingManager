CREATE TABLE news_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_key TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    occurred_on TEXT NOT NULL,
    show_id INTEGER REFERENCES shows(id),
    worker_id TEXT REFERENCES workers(id),
    is_read INTEGER NOT NULL DEFAULT 0 CHECK(is_read IN (0,1))
) STRICT;
CREATE INDEX news_date ON news_items(occurred_on DESC, id DESC);
CREATE INDEX news_unread ON news_items(is_read, occurred_on DESC, id DESC);
