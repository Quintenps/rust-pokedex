table! {
    pokemons (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "type"]
        type_ -> Jsonb,
        stats -> Jsonb,
    }
}
