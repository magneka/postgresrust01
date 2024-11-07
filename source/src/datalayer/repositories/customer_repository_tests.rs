#[cfg(test)]
mod tests {
    use crate::{customer_repository::{self, CustomerRepository}, datalayer::connection_pool::get_sql_connection, repository_tools::ParameterType};


    async fn get_repository () -> CustomerRepository {

        let pool = match get_sql_connection().await {
            Ok(pool) => pool,
            Err(error) => panic!("\n\nDB TILKOBLING FEILET: {}\n\n", error),
        };

        let customer_repository: CustomerRepository = CustomerRepository::new(pool);

        customer_repository
    }


    #[tokio::test]
    async fn test_get_all_customers() {

        let mut record_count = 0;
        let customer_repository = get_repository().await;
       
        let all_customers =customer_repository.get_all().await;
        if all_customers.is_ok() {
            record_count = all_customers.unwrap().len();
        }      
        
        assert_ne!(record_count, 0);   
    }

    
    #[tokio::test]
    async fn test_get_byid_customers() {

        let mut record_count = 0;
        let customer_repository = get_repository().await;

        let id = ParameterType::StringType("HILAA".to_string());
        let all_customers = customer_repository.get_by_id(id).await;
        if all_customers.is_ok() {
            record_count = all_customers.unwrap().len();
        } else {
            print!("{:?}", all_customers.err())
        }   
        
        assert_ne!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_get_by_comp_name_customers() {

        let mut record_count = 0;
        let customer_repository = get_repository().await;

        let id = ParameterType::StringType("Bottom-Dollar Markets".to_string());
        let all_customers = customer_repository.get_by_field("Company_Name", id).await;
        if all_customers.is_ok() {
            record_count = all_customers.unwrap().len();
        } else {
            print!("{:?}", all_customers.err())
        }   
        
        assert_ne!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_get_by_comp_name_0_customers() {

        let mut record_count = 0;
        let customer_repository = get_repository().await;

        let id = ParameterType::StringType("Finnes IKKE AS".to_string());
        let all_customers = customer_repository.get_by_field("Company_Name", id).await;
        if all_customers.is_ok() {
            record_count = all_customers.unwrap().len();
        } else {
            print!("{:?}", all_customers.err())
        }   
        
        assert_eq!(record_count, 0);   
    }
}