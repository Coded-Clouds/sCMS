use axum::{
    extract::{Extension},
    routing::{get},
    Router,
};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqliteJournalMode, SqlitePool};
use std::net::SocketAddr;
use std::fs;
use std::str::FromStr;
use anyhow::Context;

mod endpoints;
mod models;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let database_url = "sqlite://admin.db";

    let conn = SqliteConnectOptions::from_str(database_url)?
        .journal_mode(SqliteJournalMode::Wal);

    println!("{:?}", conn);

    let pool = SqlitePoolOptions::new()
        .max_connections(50)
        .connect_with(conn).await?;

    println!("{:?}", pool);

    #[derive(sqlx::FromRow)]
    struct User { fullname: String, id: i64 }

    let users = sqlx::query_as::<_, User>("Select * From User").fetch_all(&pool).await.unwrap();
    println!("{}", users[0].fullname);


    let app = Router::new()
        .route("/hello", get(root))
        .route("/users", get(endpoints::users::all_users))
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

        Ok(())

}

async fn root() -> &'static str {
    "Hello, World!"
}
 
async fn get_users(Extension(pool): Extension<SqlitePool>) {
    let users = sqlx::query_as::<_, models::user::User>("SELECT * FROM User").fetch_all(&pool).await.unwrap();
    println!("{:?}", users[0].fullname);
}
