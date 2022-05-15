use crate::models::pokemon::DbPokemon;
use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request, Response};
use std::io::Cursor;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug)]
pub enum ApiError {
    PokemonAlreadyExists(DbPokemon),
    PokemonDoesNotExist(i32),
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        match self {
            ApiError::PokemonAlreadyExists(pokemon) => {
                let msg = format!("Pokemon id: {} already exists", pokemon.id);
                Response::build().sized_body(msg.len(), Cursor::new(msg)).status(Status::Conflict).ok()
            }
            ApiError::PokemonDoesNotExist(id) => {
                let msg = format!("Pokemon id: {} not found", id);
                Response::build().sized_body(msg.len(), Cursor::new(msg)).status(Status::NotFound).ok()
            }
        }
    }
}
