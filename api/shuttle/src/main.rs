use std::fmt::format;

use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use actix_web::{web::ServiceConfig};
use sqlx::Executor;

use api_lib::health::{self};

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
        cfg.app_data(pool).configure(api_lib::health::service)
            .configure(api_lib::films::service);
    };

    Ok(config.into())
}

