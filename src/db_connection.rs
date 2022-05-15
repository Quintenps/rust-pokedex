use rocket_sync_db_pools::{diesel, database};

#[database("diesel_postgres_pool")]
pub struct PokemonDb(diesel::PgConnection);