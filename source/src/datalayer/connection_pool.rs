use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use dotenv::dotenv;
use std::env;

/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Oppretting av SQL connection                                               │
  └────────────────────────────────────────────────────────────────────────────┘
 */
pub async fn get_sql_connection () -> Result<Pool<Postgres>, Error> {

    let connection_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool: Pool<Postgres>  = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_url)
        .await?;

    Ok(pool)
}

