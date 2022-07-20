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

// DBに格納するデータとして、アヤメの測定データを定義
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct IrisMeasurement {
    pub id: Option<u64>,
    pub sepal_length: f64, // がくの長さ
    pub sepal_width: f64, // がくの幅
    pub petal_length: f64, // 花弁の長さ
    pub petal_width: f64, // 花弁の幅
    pub class: String, // 分類名
}

impl IrisMeasurement {
    const TABLE_NAME: &'static str = "iris_measurements";

    pub async fn create_table(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        pool.execute(include_str!("../sql/ddl/iris_measurements_create.sql"))
            .await
    }

    pub async fn insert(self, pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!(
            r#"INSERT INTO {} (sepal_length, sepal_width, petal_length, petal_width, class) VALUES (?, ?, ?, ?, ?)"#,
            Self::TABLE_NAME
        );
        let result = sqlx::query(&sql)
            .bind(self.sepal_length)
            .bind(self.sepal_width)
            .bind(self.petal_length)
            .bind(self.petal_width)
            .bind(self.class)
            .execute(pool)
            .await;
        result
    }

    // アヤメの種類を指定しリストを取得
    pub async fn find_by_class(
        pool: &Pool<MySql>,
        class: &str,
    ) -> Result<Vec<IrisMeasurement>, sqlx::Error> {
        let sql = format!(r#"SELECT * FROM {} WHERE class = ?"#, Self::TABLE_NAME);
        // sqlx::FromRowトレイトが実装されているのでquery_asを使うことで、
        // MySQLの行データから、IrisMeasurement構造体にデシリアライズできる
        let rows = sqlx::query_as::<_, IrisMeasurement>(&sql)
            .bind(class)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

// 詳細はWebエンジニアからみたRust第2章参照
fn get_csv_path(relative_path: &str) -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative_path)
}

// 詳細はWebエンジニアからみたRust第2章参照
pub fn read_csv(relative_path: &str) -> anyhow::Result<Vec<IrisMeasurement>> {
    let csv_path = get_csv_path(relative_path);
    let mut csv_reader = csv::Reader::from_path(csv_path)?;
    let nums = csv_reader
        .deserialize::<IrisMeasurement>()
        .filter_map(|row_result| row_result.ok())
        .collect::<Vec<_>>();
    Ok(nums)
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

    // テスト用データ生成
    fn create_fake() -> IrisMeasurement {
        IrisMeasurement {
            id: None,
            sepal_length: 3.0,
            sepal_width: 4.0,
            petal_length: 5.0,
            petal_width: 6.0,
            class: "Iris-virginica".to_string(),
        }
    }

    // テーブルの生成と初期化
    pub async fn setup_database(pool: &Pool<MySql>) {
        let _ = IrisMeasurement::create_table(pool).await.unwrap();
        let _ = truncate_table(pool, IrisMeasurement::TABLE_NAME)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn create_and_select_ok() {
        // テスト用のデータベースに接続
        let pool = create_pool(DB_STRING_TEST).await.unwrap();
        // 前回のテスト実行による副作用を初期化
        let _ = setup_database(&pool).await;
        let measurement = create_fake();
        let insert_result = measurement.insert(&pool).await.unwrap();
        // INSERT文によりデータが永続化されたか検証する
        assert_eq!(
            "MySqlQueryResult { rows_affected: 1, last_insert_id: 1 }",
            format!("{:?}", insert_result)
        );
        let actual1 = IrisMeasurement::find_by_class(&pool, "Iris-virginica")
            .await
            .unwrap();
        // 条件を満たす登録データが取得できたか検証する
        assert_eq!(actual1.len(), 1);
        let actual2 = IrisMeasurement::find_by_class(&pool, "abc").await.unwrap();
        // 条件を満たす登録データは取得できないことを検証する
        assert_eq!(actual2.len(), 0);
    }
}
