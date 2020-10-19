FROM rust:latest

RUN apt-get update && apt-get install -y libpango* libxcb-randr0*

RUN cargo install --branch dev --git https://github.com/dunmatt/thecat

# TODO: use a CMD to copy the binary somewhere.
