#[cfg(test)]
mod tests {
   
    use crate::{
        category_repository::{CategoryDto, CategoryRepository}, 
        datalayer::connection_pool::get_sql_connection, 
        repository_tools::ParameterType
    };


    async fn get_repository () -> CategoryRepository {

        let pool = match get_sql_connection().await {
            Ok(pool) => pool,
            Err(error) => panic!("\n\nDB TILKOBLING FEILET: {}\n\n", error),
        };

        let category_repository: CategoryRepository = CategoryRepository::new(pool);

        category_repository
    }


    #[tokio::test]
    async fn test_get_all_categories() {

        let mut record_count = 0;
        let category_repository = get_repository().await;
       
        let all_categorys = category_repository.get_all().await;
        if all_categorys.is_ok() {
            record_count = all_categorys.unwrap().len();
        }      
        
        assert_ne!(record_count, 0);   
    }

    
    #[tokio::test]
    async fn test_get_byid_categories() {

        let mut record_count = 0;
        let category_repository = get_repository().await;

        let id = ParameterType::Integer16(7);
        let all_categorys = category_repository.get_by_id(id).await;
        if all_categorys.is_ok() {
            record_count = all_categorys.unwrap().len();
        } else {
            print!("{:?}", all_categorys.err())
        }   
        
        assert_ne!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_get_by_name_categorys() {

        let mut record_count = 0;
        let category_repository = get_repository().await;

        let search_for = ParameterType::StringType("Produce".to_string());
        let all_categorys = category_repository.get_by_field("Category_Name", search_for).await;
        if all_categorys.is_ok() {
            record_count = all_categorys.unwrap().len();
        } else {
            print!("{:?}", all_categorys.err())
        }   
        
        assert_ne!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_get_by_comp_name_0_categorys() {

        let mut record_count = 0;
        let category_repository = get_repository().await;

        let parameter_1 = ParameterType::StringType("Finnes IKKE AS".to_string());
        let all_categorys = category_repository.get_by_field("Company_Name", parameter_1).await;
        if all_categorys.is_ok() {
            record_count = all_categorys.unwrap().len();
        } else {
            print!("{:?}", all_categorys.err())
        }   
        
        assert_eq!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_insert_category() {

        let mut _insert_result = false;
        let category_repository = get_repository().await;

        let category = CategoryDto {
            category_id:21, 
            category_name: "Mka kat".to_string(), 
            description: Some("Vi sjekker om insert fungerer".to_string()), 
            picture: None
        };

        let insert_result = category_repository.insert(&category).await;
        if insert_result.is_ok() {
            _insert_result = true;
        }   
        
        assert_eq!(_insert_result, true);   
    }
}