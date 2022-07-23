use sqlx::{
    mysql::{MySqlPoolOptions, MySqlQueryResult},
    Executor as _, MySql, Pool,
};

// 本番DB(想定)のデータベース接続文字列
pub const DB_STRING_PRODUCTION: &'static str = "mysql://user:pass@localhost:53306/production";
// rust_web_containerからアクセスする場合は上記URIをコンテナが解決できないので下記の接続文字列にする
// pub const DB_STRING_PRODUCTION: &'static str = "mysql://user:pass@mysql_container:3306/production";

// テストDB(想定)のデータベース接続文字列
pub const DB_STRING_TEST: &'static str = "mysql://user:pass@localhost:53306/test";
// rust_web_containerからアクセスする場合は上記URIをコンテナが解決できないので下記の接続文字列にする
// pub const DB_STRING_PRODUCTION: &'static str = "mysql://user:pass@mysql_container:3306/test";

// 非同期処理を実行するランタイムを作成
pub fn create_tokio_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
}

// MySQL接続のためのクライアント
// コネクションプーリングによりクライアント生成コストを削減
pub async fn create_pool(url: &str) -> Result<Pool<MySql>, sqlx::Error> {
    MySqlPoolOptions::new().connect(url).await
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<u64>,
    pub name: String, // ユーザー名
}
impl User {
    const TABLE_NAME: &'static str = "users";
    pub async fn create_table(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        pool.execute(include_str!("../sql/ddl/users_create.sql"))
            .await
    }

    pub async fn insert(self, pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!(r#"INSERT INTO {} (name) VALUES (?)"#, Self::TABLE_NAME);
        let result = sqlx::query(&sql).bind(self.name).execute(pool).await;
        result
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct UserTweet {
    pub id: Option<u64>,
    pub user_id: u64,
    pub content: String,
}
impl UserTweet {
    const TABLE_NAME: &'static str = "user_tweets";
    pub async fn create_table(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        pool.execute(include_str!("../sql/ddl/user_tweets_create.sql"))
            .await
    }
    pub async fn insert(self, pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!(
            r#"INSERT INTO {} (user_id, content) VALUES (?, ?)"#,
            Self::TABLE_NAME
        );
        let result = sqlx::query(&sql)
            .bind(self.user_id)
            .bind(self.content)
            .execute(pool)
            .await;
        result
    }
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct FollowRelations {
    pub id: Option<u64>,
    pub followee_id: u64, // フォローされる側のユーザID
    pub follower_id: u64, // フォローする側のユーザID
}
impl FollowRelations {
    const TABLE_NAME: &'static str = "follow_relations";
    pub async fn create_table(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        pool.execute(include_str!("../sql/ddl/follow_relations_create.sql"))
            .await
    }
    pub async fn insert(self, pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!(
            r#"INSERT INTO {} (followee_id, follower_id) VALUES (?, ?)"#,
            Self::TABLE_NAME
        );
        let result = sqlx::query(&sql)
            .bind(self.followee_id)
            .bind(self.follower_id)
            .execute(pool)
            .await;
        result
    }
}

// MySQLではINDEXにIF NOT EXISTSを宣言できないのでエラーハンドリングする
pub fn panic_except_duplicate_key(result: Result<MySqlQueryResult, sqlx::Error>) {
    if let Err(e) = result {
        let is_duplicate_index_error = e
            .as_database_error()
            .unwrap()
            .message()
            .starts_with("Duplicate key name");
        if !is_duplicate_index_error {
            panic!("{}", e);
        }
    };
    
}

// テーブルを生成する
// structに対するループはマクロなどを使うことを実現できるが省略
pub async fn setup_tables(pool: &Pool<MySql>) {
    panic_except_duplicate_key(User::create_table(&pool).await);
    panic_except_duplicate_key(UserTweet::create_table(&pool).await);
    panic_except_duplicate_key(FollowRelations::create_table(&pool).await);
}

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn truncate_table(
        pool: &Pool<MySql>,
        name: &str,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!("TRUNCATE TABLE {}", name);
        pool.execute(sql.as_str()).await
    }

    // テーブルの生成と初期化
    pub async fn setup_test_database() {
        let pool = create_pool(DB_STRING_TEST).await.unwrap();
        setup_tables(&pool);
        let _ = truncate_table(&pool, User::TABLE_NAME).await.unwrap();
        let _ = truncate_table(&pool, UserTweet::TABLE_NAME).await.unwrap();
        let _ = truncate_table(&pool, FollowRelations::TABLE_NAME)
            .await
            .unwrap();
    }
}
