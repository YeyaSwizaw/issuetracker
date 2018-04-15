#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate chrono;

use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;

mod schema;
mod models;

pub fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_user = env::var("DATABASE_USER").unwrap();
    let database_password = env::var("DATABASE_PASSWORD").unwrap();
    let database_host = env::var("DATABASE_HOST").unwrap();
    let database_url = format!("postgres://{}:{}@{}/issuetracker", database_user, database_password, database_host);

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    use schema::issues::dsl::*;


    let connection = establish_connection();
    let result = issues.load::<models::Issue>(&connection);

    for issue in result {
        println!("{:?}", issue);
    }
}
