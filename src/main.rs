use eframe::egui;

struct App;

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.label("Close this window with its close button.")
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "egui-close-crash",
        eframe::NativeOptions::default(),
        Box::new(|_| Ok(Box::new(App))),
    )
}
