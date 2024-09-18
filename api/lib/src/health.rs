use actix_web::{get};
#[get("/version")]
async fn get_version(
    db: actix_web::web::Data<sqlx::PgPool>,
) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {}", e),
    }
}


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}
