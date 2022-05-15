#![feature(proc_macro_hygiene, decl_macro)]
use dotenv::dotenv;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel_migrations::embed_migrations;

mod schema;
mod models;
mod routes;
mod config;
mod db_connection;
mod db;
mod errors;
mod api_result;

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

async fn migrate(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    let db = db_connection::PokemonDb::get_one(&rocket).await.expect("database connection");
    db.run(|conn| match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(e) => {
            error!("Failed to run database migrations: {:?}", e);
            Err(rocket)
        }
    })
        .await
}

#[launch]
pub fn create_rocket() -> _ {
    dotenv().ok();

    rocket::custom(config::from_env()).mount(
        "/api",
        routes![
                routes::pokemons::status,
                routes::pokemons::list_pokemons,
                routes::pokemons::create,
                routes::pokemons::create_batch,
                routes::pokemons::update,
                routes::pokemons::delete
            ],
    )
        .attach(db_connection::PokemonDb::fairing())
        .attach(AdHoc::try_on_ignite("Database Migrations", migrate))
        .register("/", catchers![errors::not_found, errors::unprocessable_entity, errors::internal_err, errors::bad_request ])
}