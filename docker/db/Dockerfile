FROM rust:latest as diesel_install

WORKDIR /usr/src/diesel_cli
RUN cargo install --root . diesel_cli --no-default-features --features postgres

FROM postgres:latest

WORKDIR /db_setup
COPY migrations ./migrations
COPY Cargo.toml .
COPY --from=diesel_install /usr/src/diesel_cli/bin/diesel /usr/bin
COPY db.sh /docker-entrypoint-initdb.d/