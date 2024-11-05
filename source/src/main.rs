/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Her er et eksenmpel på spørring mot en Azure SQL PostgreSQL database       │
  │                                                                            │
  └────────────────────────────────────────────────────────────────────────────┘
 */
//#![allow(unused_imports)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};


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
  │ Oppretting av SQL connection                                               │
  └────────────────────────────────────────────────────────────────────────────┘
 */
async fn get_sql_connection () -> sqlx::Pool<sqlx::Postgres> {

    let pool: sqlx::Pool<sqlx::Postgres>  = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://<username>:<password>@billigpg.postgres.database.azure.com/postgres")
        .await.unwrap();

    pool
}


/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Deklarasjon av data transfer object for en database tabell                 │
  │ MERK:                                                                      │
  │    Derive FromRow gjør at serialisering er mulig                           │
  │    ToPascal gjør at f.eks. db-> FirstName serialiseres til felt first_name │
  │    Debug gjør at den kan debugprintes: println!("{:?}", customer);         │
  │    Eksplisitt navngiving av dbnavn: #[sql_name="EmployeeID"]               │
  │    Option<...> brukes for felter som er nullable                           │
  └────────────────────────────────────────────────────────────────────────────┘
 */
#[derive(FromRow, Debug, Clone)]
#[sqlx(rename_all = "snake_case")]
pub struct EmployeesDto {
    #[sqlx(rename = "employee_id")]
    pub employee_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub title: Option<String>,
    pub title_of_courtesy: Option<String>,
    pub birth_date: Option<	chrono::NaiveDate>,
    pub hire_date: Option<chrono::NaiveDate>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub home_phone: Option<String>,
    pub extension: Option<String>,
    //pub photo: Option<Vec<u8>>, // <-- problem serialiseres ikke
    pub notes: Option<String>,
    pub reports_to: Option<i16>,
    pub photo_path: Option<String>,
}


/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Eksempel på kjøring av SQL uten parameter                                  │
  │ Merk serialiseringen til Vec<EmployeeDto>                                  │
  │ Det forenkler henterutinen betydelig                                       │
  └────────────────────────────────────────────────────────────────────────────┘
 */
pub async fn get_all (pool: &Pool<Postgres>) -> Vec<EmployeesDto> {

    let select_query = sqlx::query_as::<_, EmployeesDto>( 
        "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees;");

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await.unwrap();

    result

}

/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Eksempel på kjøring av SQL statement med parameter                         │
  │ Parametre navngis her (@P1)                                                │
  │ man gjør et "bind" kall for å sette parameterverdien                       │
  │ Det ser ikke ut som tiberius støtter navngitte parametre                   │  
  └────────────────────────────────────────────────────────────────────────────┘
 */
 pub async fn get_by_id (pool: &Pool<Postgres>, id: i16) -> Vec<EmployeesDto> {

    let select_query = sqlx::query_as::<_, EmployeesDto>( 
        "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees
        WHERE employee_id = $1").bind(id);

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await.unwrap();

    result
}


/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Her modifiseres SQL statementet for å angi felt man skal selectere på      │
  │ Merk at man likevel bruker parameter for selve søkeverdien                 │
  │ ellers ville man opprettet et sikkerhetshull for SQL injection             │
  │                                                                            │
  │                                                                            │
  │                                                                            │
  └────────────────────────────────────────────────────────────────────────────┘
 */
 pub async fn get_by_field (pool: &Pool<Postgres>, field_name: &str, search_for: &str) -> Vec<EmployeesDto> {

    let sql_string = "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees
        WHERE 
            __FIELDNAME__ = $1";

    let sql_string = sql_string.replace("__FIELDNAME__", field_name);

    let select_query = sqlx::query_as::<_, EmployeesDto>(&sql_string).bind(search_for);

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await.unwrap();

    result
}

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
    //let mut _client = get_sql_connection().await;
    let pool: sqlx::Pool<sqlx::Postgres> = get_sql_connection().await;
    println!("We are connected");

    println!("Fetching data");
    let all_customers: Vec<EmployeesDto> = get_all(&pool).await;
    for customer in all_customers.iter() {
        println!("{:?}", customer);
    }

    let customer_byid: Vec<EmployeesDto> = get_by_id(&pool, 1).await;
    for customer in customer_byid.iter() {
        println!("{:?}", customer);
    }    
    
    println!("\nSearch by a field");
    let field_name: &str = "Title";
    let search_for: &str = "Sales Manager";
    let customer_byfield: Vec<EmployeesDto> = get_by_field(&pool, &field_name, &search_for).await;
    for customer in customer_byfield.iter() {
         println!("{:?}", customer);
    }
      
    println!("We are now done");
}
