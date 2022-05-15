use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::schema::pokemons;

#[derive(Debug, Deserialize, Serialize)]
pub enum Type {
    Grass,
    Poison,
    Fire,
    Water,
    Bug,
    Flying,
    Normal,
    Electric,
    Fairy,
    Fighting,
    Psychic,
    Ghost,
    Dark,
    Ice,
    Rock,
    Ground,
    Steel,
    Dragon
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub sp_attack: i32,
    pub sp_defense: i32,
    pub speed: i32,
} 

#[derive(Debug, Deserialize, Serialize, Identifiable, Queryable, Insertable, AsChangeset, PartialEq)]
#[table_name = "pokemons"]
pub struct DbPokemon {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
    pub stats: serde_json::Value
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pokemon {
    pub id: i32,
    pub name: String,
    pub r#type: Vec<Type>,
    pub stats: Stats
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePokemon {
    pub name: String,
    pub r#type: Vec<Type>,
    pub stats: Stats
}

impl Pokemon {
    pub fn to_db_create_entity(self: &Self) -> DbPokemon {
        DbPokemon {
            id: self.id,
            name: self.name.clone(),
            type_: json!(self.r#type),
            stats: json!(self.stats)
        }
    }
}

impl UpdatePokemon {
    pub fn to_db_update_entity(self: &Self) -> DbPokemon {
        DbPokemon {
            id: 1, // Ignored field
            name: self.name.clone(),
            type_: json!(self.r#type),
            stats: json!(self.stats)
        }
    }
}