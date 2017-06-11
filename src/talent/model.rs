extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;

#[derive(Queryable)]
pub struct TalentModel {
    pub id:          i32,
    pub username:    String,
    pub data:        String,
    pub created_at:  String,
    pub updated_at:  String,
}

fn establish_connection() -> PgConnection {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
