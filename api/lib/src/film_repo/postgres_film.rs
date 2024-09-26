use super::{FilmRepo, FilmResult};
use shared::model::Film;
use shared::request::CreateFilmRequest;
use sqlx::postgres::PgPool;
use uuid::Uuid;

pub struct PostgresFilmRepo {
    pool: PgPool,
}

impl PostgresFilmRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilmRepo for PostgresFilmRepo {
    async fn create_film(&self, model: &CreateFilmRequest) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            INSERT INTO films (title, director, year, poster)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(&model.title)
        .bind(&model.director)
        .bind(model.year as i16)
        .bind(&model.poster)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn get_films(&self) -> FilmResult<Vec<Film>> {
        sqlx::query_as::<_, Film>(
            r#"
            SELECT * FROM films
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn get_film(&self, id: &Uuid) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
                select * from films where id = $1
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn update_film(&self, model: &Film) -> FilmResult<Film> {
        sqlx::query_as::<_, Film>(
            r#"
            UPDATE films
            SET title = $1, director = $2, year = $3, poster = $4
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(&model.title)
        .bind(&model.director)
        .bind(model.year as i16)
        .bind(&model.poster)
        .bind(&model.id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn delete_film(&self, id: &Uuid) -> FilmResult<Uuid> {
        sqlx::query_scalar::<_, Uuid>(
            r#"
            DELETE FROM films
            WHERE id = $1
            RETURNING id
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
}
