FROM rust:latest

WORKDIR /usr/src/issuetracker-api

RUN cargo install diesel_cli --no-default-features --features postgres

COPY Cargo.toml api.sh ./
COPY src ./src
COPY migrations ./migrations

ENTRYPOINT ["./api.sh"]
