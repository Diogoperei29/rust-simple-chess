use chess_gui::ChessApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 650.0])
            .with_min_inner_size([500.0, 550.0])
            .with_title("SimplyChess"),
        ..Default::default()
    };

    eframe::run_native(
        "chess-app",
        options,
        Box::new(|cc| Ok(Box::new(ChessApp::new(&cc.egui_ctx)))),
    )
}
