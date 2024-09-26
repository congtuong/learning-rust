use async_trait::async_trait;
use shared::model::Film;
use shared::request::CreateFilmRequest;
use uuid::Uuid;

pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepo: Send + Sync + 'static {
    async fn create_film(&self, id: &CreateFilmRequest) -> FilmResult<Film>;
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film>;
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn update_film(&self, id: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid>;
}

mod postgres_film;
pub use postgres_film::PostgresFilmRepo;
