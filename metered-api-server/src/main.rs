use std::{collections::HashMap, env, str::FromStr};

use dotenv::dotenv;
use rand::RngCore;
use rand_hc::Hc128Rng;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let pool = create_db_pool().await?;

    let index = warp::get()
        .and(warp::path::end())
        .map(|| "Welcome to API inc.");

    let register = warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .map(|map: HashMap<String, String>| {
            let email = map.get("email");
            match email {
                None => warp::reply::with_status(String::from("The email is required"), warp::http::StatusCode::from_u16(400).unwrap()),
                Some(email) => {
                    let mut bytes = [0; 256];
                    rand::thread_rng().fill_bytes(&mut bytes);
                    let api_key = hex::encode(bytes);
                    sqlx::query_as("INSERT INTO api_keys (email, api_key) VALUES ($1, $2)")
                        .bind(email)
                        .bind(api_key)
                        .fetch_one(&pool)
                        .await?;

                    warp::reply::with_status(format!("Your API key is '{}'", api_key), warp::http::StatusCode::from_u16(200).unwrap())
                }
            }
        });

    let not_found = warp::get().map(|| "404 page not found");

    let filters = index
        .or(register)
        .or(not_found);

    warp::serve(filters).run(([127, 0, 0, 1], 3000)).await;

    Ok(())
}

async fn create_db_pool() -> Result<Pool<Sqlite>, sqlx::Error> {
    let url = env::var("DATABASE_URL").unwrap();

    let options = SqliteConnectOptions::from_str(&url)?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;

    Ok(pool)
}
