use sqlx::{
    mysql::{MySqlPoolOptions, MySqlQueryResult},
    Executor as _, MySql, Pool,
};

// 本番DB(想定)のデータベース接続文字列
pub const DB_STRING_PRODUCTION: &'static str = "mysql://user:pass@localhost/production";

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
    pub sepal_length: f64,
    pub sepal_width: f64,
    pub petal_length: f64,
    pub petal_width: f64,
    pub class: String,
}

impl IrisMeasurement {
    const TABLE_NAME: &'static str = "iris_measurements";

    pub async fn create_table(pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
        pool.execute(include_str!("../sql/ddl/iris_measurement_create.sql"))
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

    pub async fn from_mysql_row(self, pool: &Pool<MySql>) -> Result<MySqlQueryResult, sqlx::Error> {
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

    // アヤメの種類を指定し全件取得する
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
