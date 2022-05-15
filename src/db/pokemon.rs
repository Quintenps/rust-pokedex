use crate::api_result::{ApiError, ApiResult};
use crate::diesel::ExpressionMethods;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::models::pokemon::DbPokemon;
use crate::schema::pokemons::dsl::*;
use diesel;
use diesel::pg::PgConnection;
use diesel::{insert_into, OptionalExtension};


pub fn index(conn: &PgConnection) -> Vec<DbPokemon> {
    return pokemons.order_by(id.asc()).load::<DbPokemon>(conn).expect("Error loading pokemons");
}

pub fn create(conn: &PgConnection, pokemon: DbPokemon) -> ApiResult<()> {
    let existing_db_pokemon: Option<DbPokemon> = pokemons
        .find(pokemon.id)
        .first::<DbPokemon>(conn)
        .optional()
        .expect("Error loading pokemon");

    if let Some(p) = existing_db_pokemon {
        return Err(ApiError::PokemonAlreadyExists(p));
    }

    insert_into(pokemons)
        .values(pokemon)
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error creating Pokemon");

    Ok(())
}

pub fn create_batch(conn: &PgConnection, all_pokemons: Vec<DbPokemon>) -> ApiResult<String> {
    let inserted_rows = insert_into(pokemons)
        .values(&all_pokemons)
        .on_conflict_do_nothing()
        .execute(conn)
        .expect("Error batch creating pokemons");

    Ok(format!(
        "Created {} new Pokemons from the submitted {}",
        inserted_rows,
        all_pokemons.len()
    ))
}

pub fn update(conn: &PgConnection, pokemon_id: i32, updated_pokemon: DbPokemon) -> ApiResult<()> {
    let pokemon: Option<DbPokemon> = pokemons
        .find(pokemon_id)
        .first::<DbPokemon>(conn)
        .optional()
        .expect("Error loading pokemon");

    if let None = pokemon {
        return Err(ApiError::PokemonDoesNotExist(updated_pokemon.id));
    }

    diesel::update(&pokemon.unwrap())
        .set(updated_pokemon)
        .execute(conn)
        .expect("Error loading pokemon");

    Ok(())
}

pub fn delete(conn: &PgConnection, pokemon_id: i32) -> ApiResult<()> {
    let deleted = diesel::delete(pokemons.filter(id.eq(pokemon_id))).execute(conn).expect("Error deleting pokemon");
    if deleted == 0 {
        return Err(ApiError::PokemonDoesNotExist(pokemon_id));
    }

    Ok(())
}
