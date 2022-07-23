use axum::{
    extract::{Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Serialize;
use sqlx::{MySql, Pool};

#[axum_macros::debug_handler]
pub(crate) async fn create_user(Json(_payload): Json<serde_json::Value>) -> impl IntoResponse {}

#[axum_macros::debug_handler]
pub(crate) async fn create_tweet(Json(_payload): Json<serde_json::Value>) -> impl IntoResponse {}

#[derive(Serialize)]
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
    store.spawn_cleanup_task(Duration::from_secs(60 * 60));
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8888));
    let app = Router::new()
        .route("/user", post(create_user))
        .route("/tweet", post(create_tweet))
        .route("/pages/timeline", get(get_timeline))
        .layer(Extension(pool))
        .layer(Extension(store));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[async_trait]
impl<B> FromRequest<B> for UserIdFromSession
where
    B: Send,
{
    type Rejection = (StatusCode, &'static str);
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension missing");

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let session_cookie = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(AXUM_SESSION_COOKIE_NAME));

        // return the new created session cookie for client
        if session_cookie.is_none() {
            let user_id = UserId::new();
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
            if let Some(user_id) = session.get::<UserId>("user_id") {
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
