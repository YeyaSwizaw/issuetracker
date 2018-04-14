#!/bin/bash

set -e

DATABASE_URL=postgres://$DATABASE_USER:$DATABASE_PASSWORD@db/issuetracker diesel setup
cargo run "$@"
