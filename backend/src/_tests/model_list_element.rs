use crate::model;
use model::db::init_db;
use super::{ListElementPatch, ListElementStatus, ListElementMAC};

#[tokio::test]
async fn model_list_element_create_from_path() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;
    let data_fx = ListElementPatch { 
        title: Some("Create Test".to_string()),
        status: Some(ListElementStatus::Closed.to_string()),
        ..Default::default()
    };

    let inserted_row = ListElementMAC::create_from_path("sql/db.tldb", data_fx.clone())?;
    
    const EXPECTED_ID : i32 = 13;

    assert_eq!(EXPECTED_ID, inserted_row.id, "Create Test ID");
    assert_eq!(data_fx.title.unwrap(), inserted_row.title, "Create Test Title");
    assert_eq!(data_fx.status.unwrap() , inserted_row.status, "Create Test Status");

    Ok(())
}

#[tokio::test]
async fn model_list_element_get_all_from_path() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let rows = ListElementMAC::get_all_from_path("sql/db.tldb", None)?;
    
    // last Element
    let final_row = &rows[rows.len()-1];

    assert_eq!(3, rows.len(), "Get Test Length");
    assert_eq!("List Element".to_string(), final_row.title, "Get Test Title");
    assert_eq!(ListElementStatus::Archived.to_string() , final_row.status, "Get Test Status");

    Ok(())
}

#[tokio::test]
async fn model_list_element_get_from_name_from_path_one_result() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let rows = ListElementMAC::get_from_name_from_path("sql/db.tldb", "List Element 11".to_string())?;
    
    assert_eq!(1, rows.len(), "Get From Name One Result Test Length");
    assert_eq!("List Element 11".to_string(), rows[0].title, "Get Test Title");
    assert_eq!(ListElementStatus::Closed.to_string() , rows[0].status, "Get Test Status");

    Ok(())
}

#[tokio::test]
async fn model_list_element_get_from_name_from_path_many_results() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let rows = ListElementMAC::get_from_name_from_path("sql/db.tldb", "List Element 1".to_string())?;
    
    assert_eq!(2, rows.len(), "Get From Name Many Results Test Length");
    assert_eq!("List Element 10".to_string(), rows[0].title, "Get From Name Many Results Test Title");
    assert_eq!(ListElementStatus::Open.to_string() , rows[0].status, "Get From Name Many Results Test Status");

    Ok(())
}

#[tokio::test]
async fn model_list_element_get_from_tags_from_path_one_result() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let rows = ListElementMAC::get_from_tags_from_path("sql/db.tldb", "foo bar baz".to_string())?;
    
    assert_eq!(1, rows.len(), "Get From Tags One Results Test Length");
    assert_eq!("List Element 11".to_string(), rows[0].title, "Get From Tags One Results Test Title");
    assert_eq!(ListElementStatus::Closed.to_string() , rows[0].status, "Get From Tags One Results Test Status");

    Ok(())
}

#[tokio::test]
async fn model_list_element_get_from_tags_from_path_many_results() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let rows = ListElementMAC::get_from_tags_from_path("sql/db.tldb", "foo".to_string())?;
    
    assert_eq!(3, rows.len(), "Get From Tags Many Results Test Length");
    assert_eq!("List Element".to_string(), rows[rows.len()-1].title, "Get From Tags Many Results Test Title");
    assert_eq!(ListElementStatus::Archived.to_string() , rows[rows.len()-1].status, "Get From Tags Many Results Test Status");
    
    Ok(())
}

#[tokio::test]
async fn model_list_element_update() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let data_fx = ListElementPatch { 
        title: None, 
        notes: Some("foo bar baz".to_string()), 
        tags: Some("fo bar".to_string()), 
        status: Some(ListElementStatus::Open.to_string()) 
    };


    let updated_row = ListElementMAC::update_from_path("sql/db.tldb", "List Element".to_string() , &data_fx)?;
    
    assert_eq!("List Element".to_string(), updated_row.title, "Update Test Title");
    assert_eq!(data_fx.notes , updated_row.notes, "Update Test Notes");
    assert_eq!(data_fx.tags , updated_row.tags, "Update Test Tags");
    assert_eq!(data_fx.status.unwrap() , updated_row.status, "Update Test Status");
    
    Ok(())
}

#[tokio::test]
async fn model_list_element_update_no_change() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    let data_fx = ListElementPatch { 
        title: None, 
        notes: None, 
        tags: None, 
        status: None 
    };

    let updated_row = ListElementMAC::update_from_path("sql/db.tldb", "List Element".to_string() , &data_fx)?;
    
    assert_eq!("List Element".to_string(), updated_row.title, "Update No Change Test Title");
    assert_eq!("Lipsum Orem".to_string() , updated_row.notes.unwrap(), "Update No Change Test Notes");
    assert_eq!("foo bar".to_string() , updated_row.tags.unwrap(), "Update No Change Test Tags");
    assert_eq!(ListElementStatus::Archived.to_string() , updated_row.status, "Update No Change Test Status");
    
    Ok(())
}

#[tokio::test]
async fn model_list_element_delete() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    ListElementMAC::delete_from_path("sql/db.tldb", "List Element".to_string())?;
    
    let rows = ListElementMAC::get_all_from_path("sql/db.tldb", None)?;

    assert_eq!(2, rows.len(), "Delete Test Number of elements");
    
    Ok(())
}

#[tokio::test]
async fn model_list_element_delete_nonexistant() -> Result<(), Box<dyn std::error::Error>> {

    let _db = init_db(None)?;

    //no panic here because the filter on delete will narrow to 0 elements and then do nothing
    ListElementMAC::delete_from_path("sql/db.tldb", "foo bar".to_string())?;

    let rows = ListElementMAC::get_all_from_path("sql/db.tldb", None)?;
    
    assert_eq!(3, rows.len(), "Delete Non-Existant Test Number of elements");
    
    Ok(())
}
