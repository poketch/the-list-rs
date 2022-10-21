pub mod window;
pub mod list;

// Re-export 
pub use window::run;
pub use list::List;




// main
use eframe::egui::Vec2;

pub const DEFAULT_PATH: &str = "backend/sql/db.tldb";
pub const WINDOW_SIZE: Vec2 = Vec2::new(540., 960.);