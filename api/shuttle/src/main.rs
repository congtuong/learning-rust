use std::fmt::format;

use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres(
        local_uri = "postgres://postgres:deligent_dev@localhost:5432/postgres"
    )]
    pool: String,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    println!("Creating schema");

    let pool = sqlx::PgPool::connect(&pool)
        .await
        .expect("Failed to connect to database");

    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .expect("Failed to create schema");
    let pool = actix_web::web::Data::new(pool);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(get_version);
        cfg.app_data(pool);
    };

    Ok(config.into())
}

#[get("/version")]
async fn get_version(
    db: actix_web::web::Data<sqlx::PgPool>,
) -> String {
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {}", e),
    }
}
