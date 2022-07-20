CREATE TABLE IF NOT EXISTS followee_follower_relations (
  id SERIAL,
  followee_id NOT NULL, -- フォローされる側のID
  follower_id NOT NULL, -- フォローする側のID
  FOREIGN KEY (followee_id) REFERENCES users(id) ON DELETE CASCADE -- 外部キー制約
  FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE -- 外部キー制約
);
