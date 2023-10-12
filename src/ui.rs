use std::fs::File;
use eframe::egui;
use eframe::egui::{Align2, ScrollArea, TextBuffer};
use crate::logins::Login;
use crate::password;
use egui_notify::Toasts;
use std::time::Duration;
use arboard::Clipboard;


pub struct Application {
    vec_of_rows: Vec<Row>,
    search_panel: String,
    create_new: bool,
    temp_service_name: String,
    temp_login: String,
    temp_password: String,
    toast: Toasts,
    editable: bool,
    clipboard: Option<Clipboard>,
    remove_item: Option<Row>,
}

#[derive(Clone, PartialEq)]
pub struct Row {
    pub login: Login,
    pub show_password: bool,

}


impl Default for Application {
    fn default() -> Self {
        Self { vec_of_rows: vec![], search_panel: "".to_string(), create_new: false, temp_service_name: "".to_string(), temp_login: "".to_string(), temp_password: "".to_string(), toast: Default::default(), editable: false, clipboard: None, remove_item: None }
    }
}

impl Application {
    pub fn new() -> Self {
        let mut vec_of_rows: Vec<Row> = vec![];
        let file_path = std::path::Path::new("logins.csv");
        if file_path.exists() {
            let mut rdr = csv::Reader::from_path(file_path).unwrap();

            for result in rdr.records() {
                let record = result.unwrap();

                let login = Login::new(&record[0], &record[1], &record[2]);
                vec_of_rows.push(Row { login, show_password: false });
            }
        } else {
            let file = File::create(file_path).unwrap();
            let mut wtr = csv::Writer::from_writer(file);
            wtr.write_record(&["Name_of_service","Login", "Password"]).unwrap();
        }
        let clipboard = Clipboard::new().unwrap();
        Self { vec_of_rows, search_panel: "".to_string(), create_new: false, temp_service_name: "".to_string(), temp_login: "".to_string(), temp_password: "".to_string(), toast: Default::default(), editable: false, clipboard: Some(clipboard), remove_item: None }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.toast.show(ctx);

        if self.create_new {
            egui::Window::new("Add").collapsible(false).anchor(Align2::CENTER_CENTER, [0., 0.]).auto_sized().show(ctx, |ui| {
                ui.label("Service name");
                ui.text_edit_singleline(&mut self.temp_service_name);
                ui.label("Login");
                ui.text_edit_singleline(&mut self.temp_login);
                ui.label("Password");
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.temp_password);
                    let gen_button = ui.button("🎲");
                    if gen_button.clicked() {
                        self.temp_password = password::generate_password(12usize).unwrap();
                    }
                });
                self.search_panel.clear();
                ui.horizontal(|ui| {
                    ui.add_space(160.);
                    let cancel_btn = ui.button("Cancel");
                    let create_btn = ui.button("Create");


                    if create_btn.clicked() {
                        if self.temp_login.is_empty() || self.temp_password.is_empty() || self.temp_service_name.is_empty() {
                            self.toast.warning("Some fields are empty!").set_duration(Option::from(Duration::from_secs(5)));
                        } else {
                            let new_login = Login::new(self.temp_service_name.as_str(), self.temp_login.as_str(), self.temp_password.as_str());
                            self.vec_of_rows.push(Row {
                                login: new_login,
                                show_password: false,

                            });
                            self.temp_service_name.clear();
                            self.temp_login.clear();
                            self.temp_password.clear();
                            self.create_new = false;
                        }
                    }
                    if cancel_btn.clicked() {
                        self.create_new = false;
                        self.temp_service_name.clear();
                        self.temp_login.clear();
                        self.temp_password.clear();
                    }
                });
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let search_textbox = egui::TextEdit::singleline(&mut self.search_panel);
            if self.remove_item.is_some() {
                let index = self.vec_of_rows.iter().position(|e| self.remove_item.clone().unwrap() == *e).unwrap();
                self.vec_of_rows.remove(index);
                self.remove_item = None;
            }

            ui.horizontal_top(|ui| {
                ui.add(search_textbox);
                let add_button = ui.button("Add");
                if add_button.clicked() {
                    self.create_new = true;
                }


                if self.editable {
                    let save_button = ui.button("Save");
                    if save_button.clicked() {
                        self.editable = !self.editable;
                    }
                } else {
                    let edit_button = ui.button("Edit");
                    if edit_button.clicked() {
                        self.editable = !self.editable;
                    }
                }
            });


            let search = self.search_panel.clone();

            ScrollArea::vertical().show(ui, |ui| {
                self.vec_of_rows.iter_mut()
                    .filter(|elem| { elem.login.login.to_lowercase().contains(&search.clone().to_lowercase()) || elem.login.name_of_service.to_lowercase().contains(&search.to_lowercase()) })
                    .for_each(|elem| {
                        // ui.add_space(0.6);
                        ui.group(|ui| {
                            let temp_row = elem.clone();
                            let temp_login = elem.login.login.clone();
                            let temp_password = elem.login.password.clone();
                            let login = egui::TextEdit::singleline(&mut elem.login.login).interactive(self.editable);
                            let password = egui::TextEdit::singleline(&mut elem.login.password).password(!elem.show_password).interactive(self.editable);


                            ui.horizontal(|ui| {
                                ui.label(normalize_string(elem.login.name_of_service.clone()));
                                ui.add(login);
                                let copy_login_btn = ui.button("📋");
                                ui.add(password);
                                let show_button = ui.button("👁");
                                let copy_password_btn = ui.button("📋");
                                let remove_btn = ui.button("🗑");
                                if remove_btn.clicked() {
                                    self.remove_item = Some(temp_row);
                                }

                                if copy_password_btn.clicked() {
                                    let mut clipboard = Clipboard::new().unwrap();
                                    clipboard.set_text(temp_password).unwrap();
                                }
                                if copy_login_btn.clicked() {
                                    let mut clipboard = Clipboard::new().unwrap();
                                    clipboard.set_text(temp_login).unwrap();
                                }
                                if show_button.clicked() {
                                    elem.show_password = !elem.show_password;
                                }
                            });
                        });
                    });
            })
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let mut vec_of_rows: Vec<Row> = vec![];
        let file_path = std::path::Path::new("logins.csv");
        if file_path.exists() {
            let mut rdr = csv::Reader::from_path(file_path).unwrap();

            for result in rdr.records() {
                let record = result.unwrap();
                let login = Login::new(&record[0], &record[1], &record[2]);
                vec_of_rows.push(Row { login, show_password: false });
            }
        } else {
            File::create(file_path).unwrap();
        }
        let vec_from_file = vec_of_rows.iter().map(|elem| {
            elem.login.clone()
        }).collect::<Vec<Login>>();
        let vec_from_mem = self.vec_of_rows.iter().map(|elem| {
            elem.login.clone()
        }).collect::<Vec<Login>>();

        if !do_vecs_match(&vec_from_file, &vec_from_mem) {
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            //write headers
            wtr.write_record(&["Name_of_service","Login", "Password"]).unwrap();

            vec_from_mem.iter().for_each(|elem| {
                wtr.write_record(&[elem.name_of_service.as_str(), elem.login.as_str(), elem.password.as_str()]).unwrap();
            });
            wtr.flush().unwrap();
        }
    }
}

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}


pub fn normalize_string(string: String) -> String {
    let str: String = if string.chars().count() < 15 {
        format!("{}{}", string, " ".repeat(14 - string.chars().count()).to_string())
    } else {
        format!("{}{}", string.chars().take(12).collect::<String>(), "..")
    };

    str
}
