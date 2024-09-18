use actix_web::{get, web, HttpResponse};

pub fn service(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(hello_world);
    cfg.service(get_version);
    cfg.route("/health",web::get().to(health));
}

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
// #[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().append_header(("version", "1.0.0")).finish()
}
