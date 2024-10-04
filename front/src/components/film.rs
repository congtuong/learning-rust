use crate::components::Button;
use crate::models::ButtonType;
use dioxus::prelude::*;
use log::info;
use shared::model::Film;
use crate::models::ReloadState;
use crate::api::FilmAPI;

#[component]
pub fn FilmCard<'a>(
    cx: Scope<'a>,
    film: &'a Film,
    on_edit: EventHandler<'a, MouseEvent>,
    on_delete: EventHandler<'a, MouseEvent>,
) -> Element {
    cx.render(rsx! {
        li {
            class: "flex flex-col justify-center items-center p-2 w-full md:w-1/4 m-2 bg-white shadow-md rounded-lg",
            header {
                class: "mt-4 w-full h-auto",
                img {
                    class: "max-h-80 w-auto mx-auto rounded",
                    src: "{film.poster}",
                }
            }
            section {
                h1 {
                    class: "title-font text-lg font-bold text-gray-900",
                    "{film.title}"
                }
                p {
                    class: "text-lg",
                    "{film.director}"
                }
            }

            footer {
                class: "flex justify-end items-center w-full space-x-4 mt-4",
                Button {
                    button_type: ButtonType::Primary,
                    onclick: move |_event| on_edit.call(_event),
                    "Edit",
                }
                Button {
                    button_type: ButtonType::Secondary,
                    onclick: move |_event| on_delete.call(_event),
                    "Delete",
                }
            }
        }
    })
}

#[component]
pub fn FilmList<'a>(cx: Scope<'a>, films: &'a Option<Vec<Film>>) -> Element {
    let films = match films {
        Some(films) => films,
        None => return cx.render(rsx! { "Loading..." }),
    };
    let force_get_films = use_shared_state::<ReloadState>(cx).unwrap();
    let FilmAPIInstance = use_shared_state::<FilmAPI>(cx).unwrap();
    let FilmAPIInstance = FilmAPIInstance.read().clone();

    let delete_film = move |film: Film| {
        let FilmAPIInstance = FilmAPIInstance.clone();
        let force_get_films = force_get_films.clone();

        cx.spawn({async move {

            let result = FilmAPIInstance.delete_film(&film).await;
            if let Err(_) = result {
                return;
            }
            force_get_films.write().0 = true;
        }});
    };

    cx.render(rsx! {
        ul {
            class: "flex justify-center items-center flex-wrap w-full",
            for film in films.iter() {

                FilmCard {
                    film: film,
                    on_edit: |_event| {
                        info!("on edit event");
                    },
                    on_delete: {
                        // let film = film.clone();
                        let delete_film = delete_film.clone();
                        move |_event| {
                            info!("on delete event");
                            delete_film(film.clone());
                        }

                    },
                }
            }
        }
    })
}
