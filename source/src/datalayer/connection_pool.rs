use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Oppretting av SQL connection                                               │
  └────────────────────────────────────────────────────────────────────────────┘
 */
pub async fn get_sql_connection () -> Result<Pool<Postgres>, Error> {

    let pool: Pool<Postgres>  = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://magneka:Bunnpris2012@billigpg.postgres.database.azure.com/postgres")
        .await?;

    Ok(pool)
}