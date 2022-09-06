use axum::response::IntoResponse;
use axum::http::StatusCode;

use axum::{Extension, Json};
use sqlx::SqlitePool;

use crate::{
    models::user
};


pub async fn all_users(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let users = sqlx::query_as::<_, user::User>("SELECT * FROM User").fetch_all(&pool).await.unwrap();
    (StatusCode::OK, Json(users))
}

