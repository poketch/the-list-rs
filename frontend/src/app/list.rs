
use backend::{self, model::ListElement};
use eframe::{epaint::Color32, egui::{Label, RichText, Layout, Separator}, emath::Align};
use std::{fmt::Display};
use chrono::{ NaiveDateTime, format::{DelayedFormat, StrftimeItems} };
use itertools::{Itertools};

pub const PADDING : f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

#[derive(Debug)]
struct ListElementData<'a> { 
    title: String,
    notes: String,
    tags: Vec<String>,
    status: String,
    ctime: DelayedFormat<StrftimeItems<'a>>,
    mtime: DelayedFormat<StrftimeItems<'a>>,
}

impl ListElementData <'_> {

    fn new(le: ListElement) -> Self {
        Self {
            title: le.title,
            notes: le.notes.unwrap_or_default(),
            tags: le.tags.unwrap_or_default().split_whitespace().map(|tag| tag.to_string()).collect(),
            status: le.status,
            ctime: NaiveDateTime::parse_from_str(
                le.ctime.as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .format("%H:%M %d-%m-%Y"),
            mtime: NaiveDateTime::parse_from_str(
                (le.mtime.unwrap_or(le.ctime)).as_str(), "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .format("%H:%M %d-%m-%Y"),
        }
    }
}

impl Display for ListElementData <'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f, "Title: {}, Notes: {}, Tags: {}, Status: {}, Created At: {}, Last Modified At: {}",
            self.title,
            self.notes,
            self.tags.iter().format(", "),
            self.status.to_string(),
            self.ctime,
            self.mtime,
        )
    }
}


#[derive(Debug, Default)]
pub struct List <'a>{
    list: Vec<ListElementData <'a>>
}

impl List <'_>{

    pub fn new() -> Result<Self, backend::model::Error> {
        
        Ok(Self { 
            list:
                backend::model::ListElementMAC::get_all_from_path(super::DEFAULT_PATH, None)?
                .iter().map( move |element|
                    ListElementData::new(element.to_owned())
                )
                .collect()
        })
    }

    pub fn render_list_elements(&self, ui: &mut eframe::egui::Ui) -> () {

        for ele in &self.list {
            ui.add_space(PADDING);
            
            //render LE Title
            ui.colored_label(WHITE, &ele.title);

            //render status
            ui.add_space(PADDING);
            let status = Label::new(RichText::new(&ele.status).text_style(eframe::egui::TextStyle::Button));
            ui.add(status);
            
            //render dates
            let ctime = Label::new(RichText::new(format!("Created At: {}", &ele.ctime)).text_style(eframe::egui::TextStyle::Button));
            let mtime = Label::new(RichText::new(format!("Created At: {}", &ele.mtime)).text_style(eframe::egui::TextStyle::Button));

            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                ui.add(ctime);
                ui.add_space(PADDING);
                ui.add(mtime);
            });
            
            //render tags
            
            ui.add_space(PADDING);
            
            ui.with_layout(Layout::left_to_right(Align::LEFT), |ui|{

                for tag in &ele.tags {
                    let wtag = Label::new(RichText::new(tag).text_style(eframe::egui::TextStyle::Button));
                    ui.add(wtag);
                }
            });
            
            //separator
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }




}

impl Display for List <'_>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        for element in self.list.iter() {
            write!(f, "{}\n", element)?;
        }       

        Ok(())
    }
}