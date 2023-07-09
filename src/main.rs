use eframe;
use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "infopage",
        native_options,
        Box::new(|cc| Box::new(App::new(cc)))
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas_id",
                web_options,
                Box::new(|cc| Box::new(App::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

struct App {
    c: u32
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello wasm??");
            ui.label(format!("I counted: {}", self.c));
            if ui.button("Count more!").clicked() {
                self.c += 1;
            }
        });
    }
}

impl App {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let obj = Self {
            c: 0
        };
        obj
    }
}