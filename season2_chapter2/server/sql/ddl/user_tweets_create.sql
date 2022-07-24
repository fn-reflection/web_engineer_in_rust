CREATE TABLE IF NOT EXISTS user_tweets (
  id SERIAL,
  user_id BIGINT UNSIGNED NOT NULL, -- ユーザーID
  content VARCHAR(140), -- メモ内容
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
