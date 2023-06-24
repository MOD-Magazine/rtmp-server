FROM rust:latest

WORKDIR /usr/src/mod-magazine-stream-authenticator

COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

RUN cargo install --path .

CMD ["mod-magazine-stream-authenticator"]