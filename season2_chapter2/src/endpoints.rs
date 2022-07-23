use crate::models::{FollowRelations, User, UserTweet};
use async_session::{Session, SessionStore as _};
use async_sqlx_session::MySqlSessionStore;
use axum::{
    extract::{Extension, FromRequest, Json, RequestParts, TypedHeader},
    headers::Cookie,
    http::{header::HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use sqlx::{MySql, Pool};

#[derive(Debug, serde::Deserialize)]
pub struct CreateUserParams {
    pub name: String,
}

#[axum_macros::debug_handler]
pub(crate) async fn create_user(
    Json(payload): Json<CreateUserParams>,
    pool: Extension<Pool<MySql>>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        name: payload.name,
    };
    match user.insert(&pool).await {
        Ok(res) => (
            StatusCode::OK,
            serde_json::to_string(&serde_json::json!({"messages": ["ユーザ作成成功"]})).unwrap(),
        ),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            serde_json::to_string(&serde_json::json!({"errors": ["名前が重複しています"]}))
                .unwrap(),
        ),
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct LoginParams {
    pub name: String,
}

#[axum_macros::debug_handler]
pub(crate) async fn login(
    Json(payload): Json<LoginParams>,
    pool: Extension<Pool<MySql>>,
) -> impl IntoResponse {
    dbg!(&User::find_by_name(&payload.name, &pool).await);
    match User::find_by_name(&payload.name, &pool).await {
        Ok(user) => {
            (match user {
                Some(user) => (
                    StatusCode::OK,
                    serde_json::to_string(&serde_json::json!({"messages": ["ログイン成功"]}))
                        .unwrap(),
                ),
                None => (
                    StatusCode::OK,
                    serde_json::to_string(&serde_json::json!({"errors": ["ユーザが存在しません"]}))
                        .unwrap(),
                ),
            })
        }
        Err(e) => (
            StatusCode::SERVICE_UNAVAILABLE,
            serde_json::to_string(&serde_json::json!({"errors": ["サーバーエラー発生"]})).unwrap(),
        ),
    }
}

#[axum_macros::debug_handler]
pub(crate) async fn create_tweet(Json(_payload): Json<serde_json::Value>) -> impl IntoResponse {}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Health {
    health: String,
}

#[axum_macros::debug_handler]
pub(crate) async fn get_timeline() -> impl IntoResponse {
    let health = Health {
        health: "healthy".to_string(),
    };
    (StatusCode::OK, axum::Json(health))
}

pub async fn run_server(
    pool: Pool<MySql>,
    session_store: async_sqlx_session::MySqlSessionStore,
) -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8888));
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/tweets", post(create_tweet))
        .route("/sessions", post(login))
        .route("/pages/timeline", get(get_timeline))
        .layer(Extension(pool))
        .layer(Extension(session_store));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

struct FreshUserId {
    pub user_id: u64,
    pub cookie: HeaderValue,
}

enum UserIdFromSession {
    FoundUserId(u64),
    CreatedFreshUserId(FreshUserId),
}

const AXUM_SESSION_COOKIE_NAME: &str = "axum_session";

// https://github.com/tokio-rs/axum/blob/main/examples/sessions/src/main.rs#L84より
#[axum::async_trait]
impl<B> FromRequest<B> for UserIdFromSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MySqlSessionStore>::from_request(req)
            .await
            .unwrap();

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

        // return the new created session cookie for client
        if session_cookie.is_none() {
            let user_id = 1;
            let mut session = Session::new();
            session.insert("user_id", user_id).unwrap();
            let cookie = store.store_session(session).await.unwrap().unwrap();
            return Ok(Self::CreatedFreshUserId(FreshUserId {
                user_id,
                cookie: HeaderValue::from_str(
                    format!("{}={}", AXUM_SESSION_COOKIE_NAME, cookie).as_str(),
                )
                .unwrap(),
            }));
        }
        let user_id = if let Some(session) = store
            .load_session(session_cookie.unwrap().to_owned())
            .await
            .unwrap()
        {
            if let Some(user_id) = session.get::<u64>("user_id") {
                user_id
            } else {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "No `user_id` found in session",
                ));
            }
        } else {
            return Err((StatusCode::BAD_REQUEST, "No session found for cookie"));
        };

        Ok(Self::FoundUserId(user_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        create_pool, setup_tables, FollowRelations, User, UserTweet, DB_STRING_TEST,
    };
    use sqlx::{mysql::MySqlQueryResult, Executor as _, MySql, Pool};

    pub async fn truncate_table(
        pool: &Pool<MySql>,
        name: &str,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        let sql = format!(
            "SET foreign_key_checks = 0; TRUNCATE TABLE {}; SET foreign_key_checks = 1;",
            name
        );
        pool.execute(sql.as_str()).await
    }

    // テーブルの生成と初期化
    pub async fn setup_test_database() {
        let pool = crate::models::create_pool(DB_STRING_TEST).await.unwrap();
        crate::models::setup_tables(&pool).await;
        let _ = truncate_table(&pool, FollowRelations::TABLE_NAME)
            .await
            .unwrap();
        let _ = truncate_table(&pool, UserTweet::TABLE_NAME).await.unwrap();
        let _ = truncate_table(&pool, User::TABLE_NAME).await.unwrap();
    }

    #[tokio::test]
    async fn health_ok() {
        setup_test_database().await;
        let pool = create_pool(DB_STRING_TEST).await.unwrap();
        let session_store = MySqlSessionStore::new(DB_STRING_TEST).await.unwrap();
        tokio::spawn(run_server(pool, session_store));

        let client = reqwest::Client::new();
        let response = client
            .get("http://10.10.10.11:8888/pages/timeline")
            .send()
            .await
            .unwrap();
        dbg!(&response);
    }
}
