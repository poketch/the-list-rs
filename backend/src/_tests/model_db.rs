use super::{init_db};
use fallible_iterator::FallibleIterator;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {

    // ACTION -> adjust this to use diesel?
    let db = init_db(None)?.clone().get().unwrap();
    let mut stmt = db.prepare("SELECT * from list")?;
    let rows = stmt.query([])?;
    
    // CHECK 

    const EXPECTED_SIZE : usize  = 3;

    let actual_size = rows.map(|r| r.get::<usize, i32>(0)).count()?;

    assert_eq!(EXPECTED_SIZE,actual_size , "Number of seed list elements");

    Ok(())
}