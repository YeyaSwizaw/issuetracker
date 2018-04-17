FROM rust:latest

WORKDIR /usr/src/issuetracker-api
COPY Cargo.toml Cargo.lock ./
COPY src ./src

ENTRYPOINT ["cargo", "run"]