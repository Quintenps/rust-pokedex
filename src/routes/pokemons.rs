use crate::api_result::{ApiResult};
use crate::db::{self};
use crate::db_connection::PokemonDb;
use crate::models::pokemon::{DbPokemon, Pokemon, UpdatePokemon};

use rocket::serde::json::{Json, Value};
use serde_json::json;

#[get("/status")]
pub fn status() -> &'static str {
    "Healthy"
}

#[get("/pokemons")]
pub async fn list_pokemons(db: PokemonDb) -> Value {
    let pokemons: Vec<DbPokemon> = db.run(move |conn| db::pokemon::index(conn)).await;
    json!(pokemons)
}

#[post("/pokemon/batch", format = "application/json", data = "<new_pokemons>")]
pub async fn create_batch(db: PokemonDb, new_pokemons: Json<Vec<Pokemon>>) -> ApiResult<String> {
    let pokemons: Vec<Pokemon> = new_pokemons.into_inner();
    let db_pokemons: Vec<DbPokemon> = pokemons.iter().map(|p| p.to_db_create_entity()).collect();

    db.run(move |conn| db::pokemon::create_batch(conn, db_pokemons)).await
}

#[post("/pokemon", format = "application/json", data = "<new_pokemon>")]
pub async fn create(db: PokemonDb, new_pokemon: Json<Pokemon>) -> ApiResult<()> {
    let pokemon: Pokemon = new_pokemon.into_inner();
    let db_pokemon: DbPokemon = pokemon.to_db_create_entity();

    db.run(move |conn| db::pokemon::create(conn, db_pokemon)).await
}

#[put("/pokemon/<id>", format = "application/json", data = "<update_pokemon>")]
pub async fn update(db: PokemonDb, id: i32, update_pokemon: Json<UpdatePokemon>) -> ApiResult<()> {
    let pokemon: UpdatePokemon = update_pokemon.into_inner();
    let update_db_pokemon: DbPokemon = pokemon.to_db_update_entity();

    db.run(move |conn| db::pokemon::update(conn, id, update_db_pokemon)).await
}

#[delete("/pokemon/<id>")]
pub async fn delete(db: PokemonDb, id: i32) -> ApiResult<()> {
    db.run(move |conn| db::pokemon::delete(conn, id)).await
}
