use std::fs::File;
use eframe::egui;
use eframe::egui::Shape::Path;
use crate::logins::Login;
use crate::password;

pub struct Application {
    password: String,
    vec_of_logins:Vec<Login>
}

impl Default for Application {
    fn default() -> Self {
        Self {
            password: "".to_string(),
            vec_of_logins:vec![],
        }
    }
}

impl Application {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let file_path = std::path::Path::new("logins.csv");
        if file_path.exists(){
            
        }

        Default::default()
    }
}

impl eframe::App for Application {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::CentralPanel::default().show(ctx, |ui| {



        });


    }
}
