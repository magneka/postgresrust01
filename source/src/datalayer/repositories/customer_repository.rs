#![allow(unused_imports)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query_as, Encode, Error, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};

#[allow(dead_code)]
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

        let select_query = query_as::<_, CustomersDto>( 
            "SELECT                                                                 
                     customer_id, company_name, contact_name, contact_title,             
                     address, city, region, postal_code, country, phone, fax             
                 FROM customers;");
    
        let result: Vec<CustomersDto> = select_query.fetch_all(&self.connpool).await?;
    
        Ok(result)
    }
}