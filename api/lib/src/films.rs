use crate::film_repo::FilmRepo;
use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::model::Film;
use shared::request::CreateFilmRequest;
use uuid::Uuid;

// type Repo = web::Data<Box<dyn FilmRepo>>;

pub fn service<R: FilmRepo>(cfg: &mut ServiceConfig) {
    let cors = actix_cors::Cors::default()
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header();

    cfg.service(
        web::scope("/v1")
            .wrap(cors)
            .route("/films", web::get().to(get_all_films::<R>))
            .route("/films/{id}", web::get().to(get_film::<R>))
            .route("/films", web::post().to(create_film::<R>))
            .route("/films/{id}", web::put().to(update_film::<R>))
            .route("/films/{id}", web::delete().to(delete_film::<R>)),
    );
}

async fn get_all_films<R: FilmRepo>(repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Getting all films");
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
async fn get_film<R: FilmRepo>(id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Getting film");
    match repo.get_film(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
async fn create_film<R: FilmRepo>(
    req: web::Json<CreateFilmRequest>,
    repo: web::Data<R>,
) -> HttpResponse {
    tracing::info!("Creating film");
    match repo.create_film(&req).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
async fn update_film<R: FilmRepo>(req: web::Json<Film>, repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Updating film");
    match repo.update_film(&req).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
async fn delete_film<R: FilmRepo>(id: web::Path<Uuid>, repo: web::Data<R>) -> HttpResponse {
    tracing::info!("Deleting film");
    match repo.delete_film(&id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
