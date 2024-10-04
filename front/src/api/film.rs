use log::info;
use reqwest::Error;
use shared::model::Film;
use shared::request::CreateFilmRequest;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone)]
pub struct FilmAPI {
    URL: String,
}

// web::scope("/v1")
//     .route("/films", web::get().to(get_all_films::<R>))
//     .route("/films/{id}", web::get().to(get_film::<R>))
//     .route("/films", web::post().to(create_film::<R>))
//     .route("/films/{id}", web::put().to(update_film::<R>))
//     .route("/films/{id}", web::delete().to(delete_film::<R>)),
impl FilmAPI {
    pub fn new(URL: String) -> Self {
        info!("URL: {:?}", URL);
        Self { URL: URL }
    }

    pub async fn get_films(&self) -> Result<Vec<Film>, Error> {
        info!("reqwest to URL: {}/films", self.URL);
        let url = format!("{}/films", self.URL);
        let response = reqwest::get(url).await.unwrap();
        match response.json::<Vec<Film>>().await {
            Ok(films) => {
                info!("Success: {:?}", films);
                Ok(films)
            },
            Err(e) => {
                info!("Error: {:?}", e);
                Err(e)
            }
        }
    }

    pub async fn add_film(&self, film: Film) -> Result<Film, Error> {
        let url = format!("{}/films", self.URL);
        let body = CreateFilmRequest { 
            title: film.title,
            director: film.director,
            year: film.year,
            poster: film.poster
        };

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .json(&body)
            .send()
            .await?;
        match response.json::<Film>().await {
            Ok(film) => {
                info!("Success: {:?}", film);
                Ok(film)
            },
            Err(e) => {
                info!("Error: {:?}", e);
                Err(e)
            }
        }
        
    }
    //
    // pub async fn update_film(&self, film: Film) -> Result<Film, Error> {
    //     let response = self
    //         .client
    //         .put("{self.URL}/films/{film.id}")
    //         .json(&film)
    //         .send()
    //         .await?;
    //     let film = response.json::<Film>().await?;
    //     Ok(film)
    // }
    //
    pub async fn delete_film(&self, film: &Film) -> Result<(), Error> {
        let url = format!("{}/films/{}", self.URL, film.id.to_string());
        info!("reqwest to URL: {:?}", url);
        let client = reqwest::Client::new();
        let response = client
            .delete(url)
            .send()
            .await?;
        match response.json::<String>().await {
            Ok(_) => {
                info!("Success: {:?}", film);
                Ok(())
            }
            Err(e) => {
                info!("Error: {:?}", e);
                Err(e)
            }
        }
    }
}
