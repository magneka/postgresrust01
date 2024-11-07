#![allow(unused_imports)]
#![allow(dead_code)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query, query_as, Encode, Error, Executor, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};

use super::repository_tools::{set_parameter, ParameterType};

/*
CREATE TABLE public.categories (
	category_id int2 NOT NULL,
	category_name varchar(15) NOT NULL,
	description text NULL,
	picture bytea NULL,
	CONSTRAINT pk_categories PRIMARY KEY (category_id)
);
*/

#[derive(FromRow, Debug, Clone)]
#[sqlx(rename_all = "snake_case")]
pub struct CategoryDto {                                    
    pub category_id: i16,
    pub category_name: String,
    pub description: Option<String>,
    pub picture: Option<Vec<u8>>,
}

static TABLENAME: &str = "categories";
static FIELDNAMES: &str = "category_id, category_name, description, picture";
//static FIELDNAMES: &str = "category_id, category_name, description";
static IDFIELDNAME: &str = "category_id";


pub struct CategoryRepository {
    connpool: Pool<Postgres>,
}

impl CategoryRepository {

    pub fn new(pool: Pool<Postgres>) -> Self {
        CategoryRepository {
            connpool: pool,
        }
    }
    
    pub async fn get_all (self) -> Result<Vec<CategoryDto>, Error> {

        let sql_string = format!("SELECT {} FROM {} ", FIELDNAMES, TABLENAME);
        let select_query = query_as::<_, CategoryDto>(&sql_string);
        
        let result: Vec<CategoryDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

    pub async fn get_by_id (self, id: ParameterType) -> Result<Vec<CategoryDto>, Error> {
        
        let sql_string = format!("SELECT {} FROM {} WHERE {} = $1", FIELDNAMES, TABLENAME, IDFIELDNAME);
        let mut select_query = query_as::<_, CategoryDto>(&sql_string);
        select_query = set_parameter(select_query, id);
    
        let result: Vec<CategoryDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

    pub async fn get_by_field (self, field_name: &str, search_for: ParameterType) -> Result<Vec<CategoryDto>, Error> {

        let sql_string = format!("SELECT {} FROM {} WHERE {} = $1", FIELDNAMES, TABLENAME, &field_name);            
        let mut select_query = query_as::<_, CategoryDto>(&sql_string);        
        select_query = set_parameter(select_query, search_for);
    
        let result: Vec<CategoryDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

    pub async fn insert (self, dto_record: &CategoryDto) -> Result<CategoryDto, Error> {


        let sql_string = format!("INSERT INTO {} ({}) VALUES ($1, $2, $3, $4)", TABLENAME,  FIELDNAMES);            
        
        let _query = sqlx::query(&sql_string)
        .bind(&dto_record.category_id)   
        .bind(&dto_record.category_name)   
        .bind(&dto_record.description)   
        .bind(&dto_record.picture)        
        .execute(&self.connpool).await?;

        Ok(dto_record.clone())
    }



}