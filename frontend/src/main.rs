mod app;

fn main() -> Result<(), backend::model::Error> {
    app::run();
    
    Ok(())
}