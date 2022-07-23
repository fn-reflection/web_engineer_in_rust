CREATE TABLE IF NOT EXISTS follow_relations (
  id SERIAL,
  followee_id BIGINT UNSIGNED NOT NULL, -- フォローされる側のID
  follower_id BIGINT UNSIGNED NOT NULL, -- フォローする側のID
  FOREIGN KEY (followee_id) REFERENCES users(id) ON DELETE CASCADE, -- フォロウィーユーザ削除時にフォロー関係削除
  FOREIGN KEY (follower_id) REFERENCES users(id) ON DELETE CASCADE -- フォロワーユーザ削除時にフォロー関係削除
);

CREATE UNIQUE INDEX follow_relations__follower_id__followee_id ON follow_relations (follower_id, followee_id);
