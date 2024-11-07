/*
 ┌────────────────────────────────────────────────────────────────────────────┐
 │ Her er et eksenmpel på spørring mot en Azure SQL PostgreSQL database       │
 │                                                                            │
 └────────────────────────────────────────────────────────────────────────────┘
*/
#![allow(unused_imports)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{query_as, FromRow, Pool, Postgres, Row};

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
    println!("Hello, will run queries, if dababase is online...");
    let pool = match get_sql_connection().await {
        Ok(pool) => pool,
        Err(error) => panic!("\n\nDB TILKOBLING FEILET: {}\n\n", error),
    };

    println!("We are connected");

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
    let employee_byid = employee_repository::get_by_id(&pool, 1).await;
    if employee_byid.is_ok() {
        for employee in employee_byid.unwrap().iter() {
            println!("{:?}\n", employee);
        }
    } else {
        println!("DB HENTING FEILET: {:?}\n\n", employee_byid.err());
    }

    println!("\nSearch by a field\n-----------------");
    let field_name: &str = "Title";
    let search_for: &str = "Sales Manager";
    let employee_byfield = employee_repository::get_by_field(&pool, &field_name, &search_for).await;
    if employee_byfield.is_ok() {
        for employee in employee_byfield.unwrap().iter() {
            println!("{:?}\n", employee);
        }
    } else {
        println!("DB HENTING FEILET: {:?}\n\n", employee_byfield.err());
    }

    println!("We are now done");
}
