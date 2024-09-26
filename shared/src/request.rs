use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CreateFilmRequest {
    pub title: String,
    pub director: String,
    #[cfg_attr(feature = "backend", sqlx(try_from = "i16"))]
    pub year: u16,
    pub poster: String,
}
