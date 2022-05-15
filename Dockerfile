FROM rust:1.60-buster as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:buster
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=build-env /app/target/release/pokedex /
CMD ["/pokedex"]