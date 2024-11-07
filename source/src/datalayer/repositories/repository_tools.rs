#![allow(unused_imports)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query_as, Encode, Error, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ParameterType {
    StringType (String),
    Integer16(i16),
    Integer32(i32),
}


pub fn set_parameter<T>(select_query: sqlx::query::QueryAs<'_, Postgres, T, postgres::PgArguments>, search_for: ParameterType) -> sqlx::query::QueryAs<'_, Postgres, T, postgres::PgArguments> {
    
let _select_query: sqlx::query::QueryAs<'_, Postgres, _, postgres::PgArguments> = match search_for {
    ParameterType::StringType ( s ) => select_query.bind(s.clone()),
    ParameterType::Integer16(s) => select_query.bind(s),
    ParameterType::Integer32(s) => select_query.bind(s),
};

_select_query
}