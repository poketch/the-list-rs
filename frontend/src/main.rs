mod app;

fn main() -> Result<(), backend::model::Error> {
    // app::run();

    let list = app::list::List::new()?;

    println!("{}", list);

    Ok(())
}