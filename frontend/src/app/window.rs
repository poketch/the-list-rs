#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use super::list::List;
use eframe::{NativeOptions, Frame, run_native};
use eframe::App;
use eframe::egui::{ CentralPanel, ScrollArea };
use crate::app::WINDOW_SIZE;

pub fn run() {

    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(WINDOW_SIZE);
    
    run_native("The List", win_option, Box::new( |cc| Box::new(ListApp::new(cc))))    
}

#[derive(Default)]
struct ListApp <'a>
{
 list: List <'a>,   
}

impl ListApp <'_>{

    fn new (cc: &eframe::CreationContext<'_>) -> Self {
        
        //configure do inital setup here like font families and stuff like that
        Self {
            list: List::new().unwrap()
        }
    }
}

impl App for ListApp <'_>{

    fn update(&mut self, 
        ctx: &eframe::egui::Context, 
        _frame: &mut eframe::Frame
    )
    { 
        CentralPanel::default().show( ctx, |ui|{
            ScrollArea::vertical().auto_shrink([true;2]).show(ui, |ui|{
                self.list.render_list_elements(ui);
            })
        });
    }

}



