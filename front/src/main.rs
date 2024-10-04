#![allow(non_snake_case)]
mod api;
mod components;
mod models;
use api::FilmAPI;
use components::{FilmCard, FilmList, Footer, Header};
use dioxus::prelude::*;
use dioxus_elements::u;
use log::{error, info};
use models::{FilmVisibility, ReloadState};
use shared::model::Film;
use uuid::Uuid;

fn main() {
    dioxus_web::launch(App);
}


fn App(cx: Scope) -> Element {
    wasm_logger::init(wasm_logger::Config::default().module_prefix("front"));
    let films = use_state::<Option<Vec<Film>>>(cx, || None);
    // let force_get_films = use_state::<ReloadState>(cx, || ReloadState(false));
    // use_shared_state_provider(cx, || force_get_films.clone());
    // let force_get_films = use_state::<()>(cx, || ());
    let FilmAPIInstance = FilmAPI::new(String::from("http://localhost:8000/api/v1"));
    use_shared_state_provider(cx, || FilmAPIInstance.clone());
    use_shared_state_provider(cx, || ReloadState(false));
    let force_get_films = use_shared_state::<ReloadState>(cx).unwrap();

    {
        let films = films.clone();
        let client = FilmAPIInstance.clone();
        use_effect(cx, force_get_films, |_| async move {
            let existing_films = client.get_films().await;
            match existing_films {
                Ok(data) => {
                    films.set(Some(data));
                }
                Err(_) => {
                    films.set(None);
                }
            }
        });
        // force_get_films.set(());
    }

    info!("{:?}", films);

    // let film = use_state(cx, || {
    //     Film {
    //     id: Uuid::new_v4(),
    //     title: "The Shawshank Redemption".to_string(),
    //     director: "Frank Darabont".to_string(),
    //     year: 2024,
    //     poster: "https://m.media-amazon.com/images/M/MV5BMDAyY2FhYjctNDc5OS00MDNlLThiMGUtY2UxYWVkNGY2ZjljXkEyXkFqcGc@._V1_.jpg".to_string(),
    //     created_at: Some(chrono::Utc::now()),
    //     updated_at: Some(chrono::Utc::now()),
    // }
    // });
    // info!("{:?}", film);
    cx.render(rsx! {
        main {
            class: "relative z-0 bg-blue-100 w-screen h-auto min-h-screen flex flex-col justify-start items-stretch",
            Header {            }
            section {
                class: "flex justify-center items-center w-full h-auto",
                FilmList {
                    films: films,
                }
                // FilmCard {
                //     key: "{film.id}",
                //     film: &film,
                //     on_edit: |_event| {
                //         info!("on edit event");  
                //     },
                //     on_delete: |_event| {
                //         info!("on delete event");
                //     },
                // }
            }
            Footer {}
        }
    })
}
