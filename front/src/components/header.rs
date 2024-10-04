use crate::api::FilmAPI;
use crate::components::button::Button;
use crate::models::ButtonType;
use crate::models::FilmVisibility;
use crate::models::ReloadState;
use dioxus::prelude::*;
use log::{error, info};
use shared::model::Film;
use uuid::Uuid;

// pub fn Header(cx: Scope) -> Element {
//     cx.render(rsx! {
//         header {
//             class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
//             div {
//                 class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
//                 a {
//                     class: "flex items-center md:mb-0",
//                     img {
//                         class: "bg-transparent p-2 animate-jump",
//                         src: "ferris.png",
//                         alt: "Rust Logo",
//                     }
//                     span {
//                         class: "",
//                         "Rust First Web App",
//                     }
//                 }
//             }
//         }
//     })
// }
pub fn Header(cx: Scope) -> Element {
    let FilmAPIInstance = use_shared_state::<FilmAPI>(cx).unwrap();
    let force_get_films = use_shared_state::<ReloadState>(cx).unwrap();

    let FilmAPIInstance = FilmAPIInstance.read().clone();
    // let force_get_films = force_get_films.read().clone();
    // let force_get_films = force_get_films.read().clone();
    // info!("{:?}", is_film_visible.read().0);
    cx.render(rsx!(
        header {
            class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
            div { class: "flex flex-row justify-between items-center mx-auto p-2",
                a {
                    class: "flex flex-row title-font font-medium justify-center items-center text-teal-950 m-4",
                    img {
                        class: "bg-transparent animate-jump",
                        alt: "ferris",
                        src: "ferris.png",
                        "loading": "lazy"
                    }
                    span { class: "ml-3 text-2xl", "Rusty films"}
                }
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_event| {
                        info!("Add new film event");

                        let FilmAPIInstance = FilmAPIInstance.clone();
                        let force_get_films = force_get_films.clone();

                        cx.spawn(async move {
                            // let result = FilmAPIInstance.get_films().await;
                            let result = FilmAPIInstance.add_film(
                            Film {
                                id: Uuid::new_v4(),
                                title: "The Shawshank Redemption".to_string(),
                                director: "Frank Darabont".to_string(),
                                year: 2024,
                                poster: "https://m.media-amazon.com/images/M/MV5BMDAyY2FhYjctNDc5OS00MDNlLThiMGUtY2UxYWVkNGY2ZjljXkEyXkFqcGc@._V1_.jpg".to_string(),
                                created_at: Some(chrono::Utc::now()),
                                updated_at: Some(chrono::Utc::now()),
                                }
                            ).await;
                            match result {
                                Ok(_) => {
                                    info!("Film added successfully");
                                    force_get_films.write().0 = true;
                                }
                                Err(error) => {
                                    error!("{:?}", error);
                                }
                            }
                        });
                        // let result = FilmAPIInstance.add_film(
                        //     Film {
                        //         id: Uuid::new_v4(),
                        //         title: "The Shawshank Redemption".to_string(),
                        //         director: "Frank Darabont".to_string(),
                        //         year: 2024,
                        //         poster: "https://m.media-amazon.com/images/M/MV5BMDAyY2FhYjctNDc5OS00MDNlLThiMGUtY2UxYWVkNGY2ZjljXkEyXkFqcGc@._V1_.jpg".to_string(),
                        //         created_at: Some(chrono::Utc::now()),
                        //         updated_at: Some(chrono::Utc::now()),
                        //     }
                        // ).await;
                    },
                    "Add new film",
                }
            }
        }
    ))
}
