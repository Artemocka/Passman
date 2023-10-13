use eframe;
use eframe::egui;

mod ui;
mod password;
mod logins;

use ui::Application;

fn main() -> eframe::Result<()> {

    let mut native_options = eframe::NativeOptions::default();
    native_options.min_window_size= Some(egui::Vec2::new(820.,600.));
    eframe::run_native(
        "Passman",
        native_options,
        Box::new(|_cc| Box::new(Application::new())),
    )
}


