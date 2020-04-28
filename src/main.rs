use crate::db::{build_pool, fetch_activities};
use std::env;

mod db;

#[async_std::main]
async fn main() {
    // Create a connection pool
    let db_url = env::var("DATABASE_URL").expect("Must Set DATABASE_URL");
    let num_cons = 5;

    let pool = build_pool(&db_url, num_cons)
        .await
        .expect("Failed to initialize Postgres Pool");

    // Make a simple query to return the given parameter
    let row = fetch_activities(&pool).await;

    println!("{:?}", row)
}
