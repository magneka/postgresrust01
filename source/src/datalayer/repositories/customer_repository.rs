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

    pub async fn insert (self, dto_record: &CustomersDto) -> Result<CustomersDto, Error> {

        let sql_string = format!("INSERT INTO {} ({}) 
        VALUES $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ", TABLENAME,  FIELDNAMES);            
    
        //let new_id: (i16,) = sqlx::query_as(&sql_string)
        let _qresult = sqlx::query(&sql_string)
        .bind(&dto_record.customer_id)   
        .bind(&dto_record.company_name)   
        .bind(&dto_record.contact_name)   
        .bind(&dto_record.contact_title)   
        .bind(&dto_record.address)               
        .bind(&dto_record.city)        
        .bind(&dto_record.region)        
        .bind(&dto_record.postal_code)        
        .bind(&dto_record.country)        
        .bind(&dto_record.phone)        
        .bind(&dto_record.fax)   
        .execute(&self.connpool).await?;

        let result = dto_record.clone();
        //result.category_id = new_id.0;

        Ok(result)
    }

    pub async fn update (self, dto_record: &CustomersDto) -> Result<u64, Error> {

        let sql_string = format!(
            "UPDATE 
                {} 
            SET 
                company_name = $2, 
                contact_name = $3, 
                contact_title = $4, 
                address = $5, 
                city = $6, 
                region = $7, 
                postal_code = $8, 
                country = $9, 
                phone = $10, 
                fax = $11
            WHERE 
                {} = $1;
            ",        
            TABLENAME,  IDFIELDNAME);            
        
        let results = sqlx::query(&sql_string)
        .bind(&dto_record.customer_id)  
        .bind(&dto_record.company_name)   
        .bind(&dto_record.contact_name)   
        .bind(&dto_record.contact_title)   
        .bind(&dto_record.address)               
        .bind(&dto_record.city)        
        .bind(&dto_record.region)        
        .bind(&dto_record.postal_code)        
        .bind(&dto_record.country)        
        .bind(&dto_record.phone)        
        .bind(&dto_record.fax)        
        .execute(&self.connpool).await?;

        Ok(results.rows_affected())
    }

    pub async fn delete (self, id: &i16) -> Result<u64, Error> {

        let sql_string = format!(
            "DELETE FROM 
                {} 
            WHERE 
                {} = $1;
            ",        
            TABLENAME,  IDFIELDNAME);            
        
        let results = sqlx::query(&sql_string)
        .bind(&id)           
        .execute(&self.connpool).await?;

        Ok(results.rows_affected())
    }
}