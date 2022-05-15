CREATE TABLE pokemons (
  id integer NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  type jsonb NOT NULL,
  stats jsonb NOT NULL
)