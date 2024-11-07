#![allow(unused_imports)]
#![allow(dead_code)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query_as, Encode, Error, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};

use super::repository_tools::{set_parameter, ParameterType};

#[derive(FromRow, Debug, Clone)]
#[sqlx(rename_all = "snake_case")]
pub struct CustomersDto {                                    
    pub customer_id: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub contact_title: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,                                         
    pub phone: Option<String>,                                            
    pub fax: Option<String>,
}

static TABLENAME: &str = "customers";
static FIELDNAMES: &str = "
    customer_id, company_name, contact_name, contact_title, 
    address, city, region, postal_code, country, phone, fax";
static IDFIELDNAME: &str = "customer_id";


pub struct CustomerRepository {
    connpool: Pool<Postgres>,
}

impl CustomerRepository {

    pub fn new(pool: Pool<Postgres>) -> Self {
        CustomerRepository {
            connpool: pool,
        }
    }
    
    pub async fn get_all (self) -> Result<Vec<CustomersDto>, Error> {

        let sql_string = format!("SELECT {} FROM {}", FIELDNAMES, TABLENAME);
        let select_query = query_as::<_, CustomersDto>(&sql_string);
        
        let result: Vec<CustomersDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

    pub async fn get_by_id (self, id: ParameterType) -> Result<Vec<CustomersDto>, Error> {
        
        let sql_string = format!("SELECT {} FROM {} WHERE {} = $1", FIELDNAMES, TABLENAME, IDFIELDNAME);
        let mut select_query = query_as::<_, CustomersDto>(&sql_string);
        select_query = set_parameter(select_query, id);
    
        let result: Vec<CustomersDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

    pub async fn get_by_field (self, field_name: &str, search_for: ParameterType) -> Result<Vec<CustomersDto>, Error> {

        let sql_string = format!("SELECT {} FROM {} WHERE {} = $1", FIELDNAMES, TABLENAME, &field_name);            
        let mut select_query = query_as::<_, CustomersDto>(&sql_string);        
        select_query = set_parameter(select_query, search_for);
    
        let result: Vec<CustomersDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }

}