use eframe;

mod ui;
mod password;
mod logins;

use ui::Application;

fn main() -> eframe::Result<()> {

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Passman",
        native_options,
        Box::new(|cc| Box::new(Application::new(cc))),
    )
}


