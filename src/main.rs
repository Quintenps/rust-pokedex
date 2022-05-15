use pokedex;
use rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    pokedex::create_rocket().launch().await
}