#![allow(unused_imports)]
#![allow(dead_code)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query, query_as, query_as_unchecked, Encode, Error, Executor, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};

use super::repository_tools::{set_parameter, ParameterType};
use crate::category_repository::CategoryDto;

pub struct GenericRepository {
    connpool: Pool<Postgres>,
}

impl GenericRepository {

    pub fn new(pool: Pool<Postgres>) -> Self {
        GenericRepository {
            connpool: pool,
        }
    }

    pub async fn select_by_id(self, id: i16) -> Option<CategoryDto> {

        let account = sqlx::query_as!(
            CategoryDto,
            "select * from categories where category_id = $1",
            id  //1i16
        )
        .fetch_one(&self.connpool)
        .await.unwrap();

        Some(account)
    }
    
    // pub async fn get_all<T> (self, _sql_string: &String) -> Result<Vec<T>, Error> {

    //     let sql_string = format!("SELECT {} FROM {} ", "FIELDNAMES", "TABLENAME");
    //     let select_query: Vec<T> = query_as_unchecked!(user, "SELECT * from Categories");
        
    //     let result: Vec<T> = select_query.fetch_all(&self.connpool).await?;
    
    //     Ok(result)
    // }

}
