/*
 ┌────────────────────────────────────────────────────────────────────────────┐
 │ Her er et eksenmpel på spørring mot en Azure SQL PostgreSQL database       │
 │                                                                            │
 └────────────────────────────────────────────────────────────────────────────┘
*/
#![allow(unused_imports)]
use std::borrow::Borrow;

use category_repository::{CategoryDto, CategoryRepository};
use customer_repository::CustomerRepository;
use repository_tools::ParameterType;
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{query_as, Either, FromRow, Pool, Postgres, Row};
use colored::Colorize;
use dotenv::dotenv;

mod datalayer;
use crate::datalayer::connection_pool::get_sql_connection;
use crate::datalayer::repositories::*;

/*
 ┌─────────────────────────────────────────────────────────────────────────────┐
 │ OPPGAVE:                                                                    │
 │ Lag DTO og rutiner for å hente data fra Customers tabellen                  │
 │                                                                             │
 │     CREATE TABLE customers (                                                │
 │         customer_id varchar(5) NOT NULL,                                    │
 │         company_name varchar(40) NOT NULL,                                  │
 │         contact_name varchar(30) NULL,                                      │
 │         contact_title varchar(30) NULL,                                     │
 │         address varchar(60) NULL,                                           │
 │         city varchar(15) NULL,                                              │
 │         region varchar(15) NULL,                                            │
 │         postal_code varchar(10) NULL,                                       │
 │         country varchar(15) NULL,                                           │
 │         phone varchar(24) NULL,                                             │
 │         fax varchar(24) NULL,                                               │
 │         CONSTRAINT pk_customers PRIMARY KEY (customer_id)                   │
 │                                                                             │
 │     SELECT                                                                  │
 │         customer_id, company_name, contact_name, contact_title,             │
 │         address, city, region, postal_code, country, phone, fax             │
 │     FROM customers;                                                         │
 └─────────────────────────────────────────────────────────────────────────────┘
*/

/*
 ┌────────────────────────────────────────────────────────────────────────────┐
 │ Hovedrutine                                                                │
 │ Må bruke Tokio eller tilsvarende for å implementere async trait (interface)│
 │ Merk bruk av {:?} som der Debug printing                                   │
 └────────────────────────────────────────────────────────────────────────────┘
*/
#[tokio::main]
async fn main() {

    dotenv().ok();

    println!("{}", "Hello, will run queries, if dababase is online...".yellow());
    let pool = match get_sql_connection().await {
        Ok(pool) => pool,
        Err(error) => panic!("\n\nDB TILKOBLING FEILET: {}\n\n", error.to_string().red()),
    };

    println!("We are connected");

    /*
    println!("Fetching data: select all\n-------------------------");
    let all_employees = employee_repository::get_all(&pool).await;
    if all_employees.is_ok() {
        for employee in all_employees.unwrap().iter() {
            println!("{:?}\n", employee);
        }
    } else {
        println!("DB HENTING FEILET: {:?}\n\n", all_employees.err());
    }

    println!("\nSearch by id\n------------");
    let id = ParameterType::Integer16(1);
    let employee_byid = employee_repository::get_by_id(&pool, id).await;
    if employee_byid.is_ok() {
        for employee in employee_byid.unwrap().iter() {
            println!("{:?}\n", employee);
        }
    } else {
        println!("DB HENTING FEILET: {:?}\n\n", employee_byid.err());
    }

    println!("\nSearch by a field\n-----------------");
    let field_name: &str = "Title";
    //let search_for: &str = "Sales Manager";
    let param = ParameterType::StringType("Sales Manager".to_string());
    let employee_byfield = employee_repository::get_by_field(&pool, &field_name, param).await;
    if employee_byfield.is_ok() {
        for employee in employee_byfield.unwrap().iter() {
            println!("{:?}\n", employee);
        }
    } else {
        println!("DB HENTING FEILET: {:?}\n\n", employee_byfield.err());
    }

    let category_repository: CategoryRepository = CategoryRepository::new(pool);

    */

    let mut category_repository =  &CategoryRepository::new(pool);

    //let category_name = "Mka kat";
    //let description = "Vi sjekker om insert fungerer";
    //let insert_result = category_repository.insert(&category_name, &description).await;

    let mut category = CategoryDto { 
        category_id: 0, 
        category_name: "MKA TEST".to_string(), 
        description: Some("Did it return id?".to_string()), 
        picture: None,        
    };
    print!("Category before insert: {:?}\n", category);
    
    let insert_result = &category_repository.clone().insert(&category).await;
    match insert_result {
        Ok(e) => {
            println!("insert ok, id= {:?}", e.category_id);  
            category = e.clone();         
        }
        Err(e) => {
            println!("**** insert feilet fordi:\n{:?}", e);
        }
    };
    print!("Category after insert: {:?}\n", category);
    //let category.category_id = insert_result.as_ref().unwrap().category_id;
    //let inserted = insert_result.as_ref().unwrap();
    //print!("Inserted record: {:?}", inserted);
    
    let category_to_update = category_repository::CategoryDto {
        category_id: category.category_id, 
        category_name: "My Category".to_string(), 
        description: Some("Juhuu, was uptdated!!??, gone?".to_string()), 
        picture: None
    };
    
    let update_result = &category_repository.clone().update(&category_to_update.clone()).await;
    let rows_affected: u64 = *update_result.as_ref().unwrap();
    println!("updated {:?} rows.", rows_affected);

    let delete_result = &category_repository.clone().delete(&category.category_id).await;
    let rows_affected: u64 = *delete_result.as_ref().unwrap();
    println!("Deleted byid: {}, {:?} rows.", &category.category_id, rows_affected);
        
    println!("We are now done");
}
