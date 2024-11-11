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
            category_id: 9, 
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

    #[tokio::test]
    async fn test_update_category() {

        let mut record_count = 0;
        let category_repository = get_repository().await;

        let category = CategoryDto {
            category_id:9, 
            category_name: "MKA UPDATE".to_string(), 
            description: Some("Insert må ha fungert,".to_string()), 
            picture: None
        };

        let updated_records = category_repository.update(&category).await;
        if updated_records.is_ok() {
            record_count = updated_records.unwrap();
        } else {
            print!("{:?}", updated_records.err())
        }    
        
        assert_eq!(record_count, 0);   
    }

    #[tokio::test]
    async fn test_insert_update_delete() {

        // Opprette repo med connection
        let mut category_repository = get_repository().await;

        // Ny categori
        let mut category = CategoryDto { 
            category_id: 0, 
            category_name: "MKA TEST".to_string(), 
            description: Some("Did it return id?".to_string()), 
            picture: None,        
        };
        print!("Category before insert: {:?}\n", category);

        // Insert og sjekk at vi fikk ny ID
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

        // Hent categori basert på ny ID for å se at den er der
        let mut record_count = 0;
        let id = ParameterType::Integer16(category.category_id);
        let all_categorys = &category_repository.clone().get_by_id(id.to_owned()).await;
        if all_categorys.is_ok() {
            record_count = all_categorys.as_ref().unwrap().len();
        } else {
            print!("{:?}", all_categorys.as_ref().err())
        }
        print!("Hentet {} record by id {:?}\n", record_count, all_categorys);

        // Lag en ny categori med samme id for å oppdatere db
        let category_to_update = CategoryDto {
            category_id: category.category_id, 
            category_name: "My Category".to_string(), 
            description: Some("Juhuu, was uptdated!!??, gone?".to_string()), 
            picture: None
        };

        // Oppdtater og sjekk hvor mange rader som ble oppdatert
        let update_result = &category_repository.clone().update(&category_to_update.clone()).await;
        let rows_affected: u64 = *update_result.as_ref().unwrap();
        println!("updated {:?} rows.", rows_affected);

        // Hent ut by id igjen for å sjekke at den er oppdatert
        let all_categorys = &category_repository.clone().get_by_id(id).await;
        if all_categorys.is_ok() {
            record_count = all_categorys.as_ref().unwrap().len();
        } else {
            print!("{:?}", all_categorys.as_ref().err())
        }
        print!("Hentet {} record by id {:?}\n", record_count, all_categorys);

        // Så rydder vi til slutt med å slette posten
        let delete_result = &category_repository.clone().delete(&category.category_id).await;
        let rows_affected: u64 = *delete_result.as_ref().unwrap();
        println!("Deleted byid: {}, {:?} rows.", &category.category_id, rows_affected);

    }



    /*
    
    #[tokio::test]
    async fn test_insert_category_using_macro() {

        let mut _insert_result = false;
        let category_repository = get_repository().await;

        let category = CategoryDto {
            category_id:21, 
            category_name: "Mka kat".to_string(), 
            description: Some("Vi sjekker om insert fungerer".to_string()), 
            picture: None
        };


        let category_name = "Mka kat";
        let description = "Vi sjekker om insert fungerer";

        let insert_result = category_repository.insert_by_macro(&category_name, &description).await;
        if insert_result.is_ok() {
            _insert_result = true;
        }   
        
        assert_eq!(_insert_result, true);   
    }
    */
}