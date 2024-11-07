#![allow(unused_imports)]
use sqlx::postgres::{self, PgPoolOptions, PgRow};
use sqlx::{query_as, Encode, Error, FromRow, Pool, Postgres, Row};
use sqlx::types::chrono::{DateTime, Utc};

use super::repository_tools::{set_parameter, ParameterType};

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
#[allow(dead_code)]
#[derive(FromRow, Debug, Clone)]
#[sqlx(rename_all = "snake_case")]
pub struct EmployeesDto {
    #[sqlx(rename = "employee_id")]
    pub employee_id: i16,
    pub first_name: String,
    pub last_name: String,
    pub title: Option<String>,
    pub title_of_courtesy: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub hire_date: Option<chrono::NaiveDate>,
    pub region: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub home_phone: Option<String>,
    pub extension: Option<String>,
    pub photo: Option<Vec<u8>>, // <-- problem serialiseres ikke
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
pub async fn get_all (pool: &Pool<Postgres>) -> Result<Vec<EmployeesDto>, Error> {

    let select_query = query_as::<_, EmployeesDto>( 
        "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees;");

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await?;

    Ok(result)
}

/* 
  ┌────────────────────────────────────────────────────────────────────────────┐
  │ Eksempel på kjøring av SQL statement med parameter                         │
  │ Parametre navngis her (@P1)                                                │
  │ man gjør et "bind" kall for å sette parameterverdien                       │
  │ Det ser ikke ut som tiberius støtter navngitte parametre                   │  
  └────────────────────────────────────────────────────────────────────────────┘
 */
 pub async fn get_by_id (pool: &Pool<Postgres>, id: ParameterType) -> Result<Vec<EmployeesDto>, Error> {

    let sql_string = "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees
        WHERE employee_id = $1";

    let mut select_query = query_as::<_, EmployeesDto>(&sql_string);
    select_query = set_parameter(select_query, id);

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await?;

    Ok(result)
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
pub async fn get_by_field (pool: &Pool<Postgres>, field_name: &str, search_for: ParameterType) -> Result<Vec<EmployeesDto>, Error> {

    let sql_string = "SELECT employee_id, last_name, first_name, title, title_of_courtesy, 
            birth_date, hire_date, address, city, region, postal_code, country, home_phone, 
            extension, photo, notes, reports_to, photo_path
        FROM employees
        WHERE 
            __FIELDNAME__ = $1";

    let sql_string = sql_string.replace("__FIELDNAME__", field_name);

    let mut select_query = query_as::<_, EmployeesDto>(&sql_string);
    select_query = set_parameter(select_query, search_for);

	let result: Vec<EmployeesDto> = select_query.fetch_all(pool).await?;

    Ok(result)
}
/*
pub fn set_parameter(select_query: sqlx::query::QueryAs<'_, Postgres, EmployeesDto, postgres::PgArguments>, search_for: ParameterType) -> sqlx::query::QueryAs<'_, Postgres, EmployeesDto, postgres::PgArguments> {
    let select_query: sqlx::query::QueryAs<'_, Postgres, _, postgres::PgArguments> = match search_for {
        ParameterType::StringType ( s ) => select_query.bind(s.clone()),
        ParameterType::Integer16(s) => select_query.bind(s),
        ParameterType::Integer32(s) => select_query.bind(s),
    };
    select_query
}
*/