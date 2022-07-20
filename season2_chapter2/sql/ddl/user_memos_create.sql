CREATE TABLE IF NOT EXISTS memos (
  id SERIAL, -- メモID
  user_id NOT NULL, -- ユーザーID
  memo VARCHAR(140), -- メモ
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE -- 外部キー制約
);