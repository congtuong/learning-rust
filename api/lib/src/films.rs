use actix_web::{web::{self, ServiceConfig}, HttpResponse};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/films", web::get().to(get_all_films))
            .route("/films/{id}", web::get().to(get_film))
            .route("/films", web::post().to(create_film))
            .route("/films/{id}", web::put().to(update_film))
            .route("/films/{id}", web::delete().to(delete_film))
    );
}

async fn get_all_films() -> HttpResponse {
    tracing::info!("Getting all films");
    HttpResponse::Ok().finish()
}
async fn get_film() -> HttpResponse {
    tracing::info!("Getting film");
    HttpResponse::Ok().finish()
}
async fn create_film() -> HttpResponse {
    tracing::info!("Creating film");
    HttpResponse::Ok().finish()
}
async fn update_film() -> HttpResponse {
    tracing::info!("Updating film");
    HttpResponse::Ok().finish()
}
async fn delete_film() -> HttpResponse {
    tracing::info!("Deleting film");
    HttpResponse::Ok().finish()
}