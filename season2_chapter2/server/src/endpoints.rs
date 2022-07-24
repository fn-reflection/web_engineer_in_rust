// データモデルの読み込み
use crate::models::{timeline, FollowRelation, User, UserTweet};
use async_session::{Session, SessionStore as _};
// セッション情報をMySQLに保存するライブラリ
use async_sqlx_session::MySqlSessionStore;
use axum::{
    extract::{Extension, FromRequest, Json, RequestParts},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
// クライアントクッキーを制御する便利なライブラリ
use axum_extra::extract::cookie::{Cookie, CookieJar};
use sqlx::{MySql, Pool};
use std::sync::Arc;

// ユーザ新規作成APIのリクエストJSONのスキーマ
#[derive(serde::Deserialize)]
pub struct CreateUserParams {
    pub name: String,
}

// ユーザ新規作成API
pub(crate) async fn create_user(
    Json(payload): Json<CreateUserParams>,
    arc_pool: Extension<Arc<Pool<MySql>>>,
) -> impl IntoResponse {
    let user = User {
        id: None,
        name: payload.name,
    };
    // ユーザ登録を試みる
    match user.insert(&arc_pool).await {
        // 成功したらHTTPステータスコード201を返す
        Ok(_res) => StatusCode::CREATED,
        // 失敗したらHTTPステータスコード400を返す
        // ユーザ名重複やサーバ接続エラーなど
        // より精緻にステータスコードを分けることもできる
        Err(_e) => StatusCode::BAD_REQUEST,
    }
}

// ログインAPI
#[derive(serde::Deserialize)]
pub struct CreateSessionParams {
    pub name: String,
}

pub(crate) async fn create_session(
    Json(payload): Json<CreateSessionParams>,
    arc_pool: Extension<Arc<Pool<MySql>>>,
    session_store: Extension<MySqlSessionStore>,
    cookie_jar: CookieJar,
) -> impl IntoResponse {
    // リクエストされた名前が存在するか調べる
    match User::find_by_name(&payload.name, &arc_pool).await {
        Ok(user) => match user {
            // ユーザー名が存在するならログイン処理
            Some(user) => {
                let mut session = Session::new();
                session.expire_in(std::time::Duration::from_secs(86400));
                session.insert("user_id", user.id).unwrap();
                // MySQLにセッション保存を試みる
                match session_store.store_session(session).await {
                    Ok(cookie_value) => Ok((
                        StatusCode::CREATED,
                        // 成功したらSet-Cookieレスポンスヘッダを通じてクッキーを更新
                        cookie_jar.add(Cookie::new(AXUM_SESSION_COOKIE_KEY, cookie_value.unwrap())),
                    )),
                    Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
                }
            }
            // ユーザー名が存在しない場合
            None => Err(StatusCode::BAD_REQUEST),
        },
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateUserTweetParams {
    pub content: String,
}

// ツイート作成API
pub(crate) async fn create_user_tweet(
    Json(payload): Json<CreateUserTweetParams>,
    arc_pool: Extension<Arc<Pool<MySql>>>,
    session: CurrentSession,
) -> impl IntoResponse {
    // セッションからuser_idを取得する
    match session.0.get::<u64>("user_id") {
        Some(user_id) => {
            let tweet = UserTweet {
                id: None,
                user_id,
                content: payload.content,
            };
            match tweet.insert(&arc_pool).await {
                Ok(_) => Ok(StatusCode::CREATED),
                Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
            }
        }
        // セッションからuser_idを復元できない場合
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

#[derive(serde::Deserialize)]
pub struct CreateFollowRelationParams {
    pub name: String,
}

// フォローAPI
pub(crate) async fn create_follow_relation(
    Json(payload): Json<CreateFollowRelationParams>,
    arc_pool: Extension<Arc<Pool<MySql>>>,
    session: CurrentSession,
) -> impl IntoResponse {
    match session.0.get::<u64>("user_id") {
        Some(user_id) => {
            let result = User::find_by_name(&payload.name, &arc_pool).await;
            match result {
                Ok(followee) => match followee {
                    Some(followee) => {
                        let follow_relation = FollowRelation {
                            id: None,
                            followee_id: followee.id.unwrap(),
                            follower_id: user_id,
                        };
                        match follow_relation.insert(&arc_pool).await {
                            Ok(_) => Ok(StatusCode::CREATED),
                            Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
                        }
                    }
                    None => Err(StatusCode::BAD_REQUEST),
                },
                Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub(crate) async fn get_timeline(
    arc_pool: Extension<Arc<Pool<MySql>>>,
    session: CurrentSession,
) -> impl IntoResponse {
    match session.0.get::<u64>("user_id") {
        Some(user_id) => match timeline(user_id, &arc_pool).await {
            Ok(tweets) => Ok(axum::Json(tweets)),
            Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
        },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn run_server(
    arc_pool: Arc<Pool<MySql>>,
    session_store: MySqlSessionStore,
) -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8888));
    let app = Router::new()
        .route("/api/users", post(create_user))
        .route("/api/sessions", post(create_session))
        .route("/api/user_tweets", post(create_user_tweet))
        .route("/api/follow_relations", post(create_follow_relation))
        .route("/api/pages/timeline", get(get_timeline))
        .layer(Extension(arc_pool))
        .layer(Extension(session_store));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

pub struct CurrentSession(Session);
const AXUM_SESSION_COOKIE_KEY: &str = "axum_session";
// https://github.com/tokio-rs/axum/blob/main/examples/sessions/src/main.rsを改変
// axumのカスタムextractorを定義
// クッキーに格納されたセッションキーからセッションデータを復元する
#[axum::async_trait]
impl<B> FromRequest<B> for CurrentSession
where
    B: Send,
{
    type Rejection = StatusCode;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // MySQLセッションストアを参照する
        let Extension(store) = Extension::<MySqlSessionStore>::from_request(req)
            .await
            .unwrap();
        // ブラウザから送信されたクッキーを参照する
        let cookie = CookieJar::from_request(req).await.unwrap();
        // クッキーからセッションキーを取得
        let session_id = cookie
            .get(AXUM_SESSION_COOKIE_KEY)
            .map(|cookie| cookie.value())
            .unwrap_or("")
            .to_string();
        // セッションキーからセッションデータを復元する
        let session_data = store.load_session(session_id).await;
        match session_data {
            Ok(session_data) => match session_data {
                // セッションデータが存在＝セッションデータを返す
                Some(session_data) => Ok(CurrentSession(session_data)),
                // セッションデータが存在しない＝ログインできていない
                None => Err(StatusCode::UNAUTHORIZED),
            },
            // RDBとの接続が切れている可能性がある、500を返す
            Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
        }
    }
}
