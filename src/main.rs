use eframe::egui;
use rb_term::gui::RtGui;
use rb_term::pty::RtPty;

fn font_definitions() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "Hack".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/hack/Hack-Regular.ttf")),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "Hack".to_owned());

    fonts
}

fn main() {
    std::env::remove_var("TERM");

    let subscriber = tracing_subscriber::FmtSubscriber::new();

    tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");

    let shell = "/bin/bash";
    std::env::set_var("PS1", "$ ");
    let pty = RtPty::new(shell);

    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Rabbit Term",
        native_options,
        Box::new(move |cc| {
            let style = egui::Style {
                visuals: egui::Visuals {
                    override_text_color: Some(egui::Color32::WHITE),
                    extreme_bg_color: egui::Color32::BLACK,
                    ..egui::Visuals::dark()
                },

                ..egui::Style::default()
            };

            cc.egui_ctx.set_style(style);
            cc.egui_ctx.set_fonts(font_definitions());
            Box::new(RtGui::new(cc, pty))
        }),
    );
}
