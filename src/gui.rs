use crate::log::{event, Level};
use crate::pty::Pty;
use crate::pty::PtyActionError;
use crate::pty::RtPty;
use eframe::egui;

pub struct RtGui {
    pub pty: RtPty,
}

impl RtGui {
    pub fn new(_cc: &eframe::CreationContext<'_>, pty: RtPty) -> Self {
        Self { pty }
    }

    fn handle_input(&mut self, i: &egui::InputState) {
        for event in &i.events {
            let text = match event {
                egui::Event::Text(s) => s,
                egui::Event::Key {
                    key, pressed: true, ..
                } => match key {
                    egui::Key::Enter => "\n",
                    egui::Key::Backspace => "\x7f",
                    _ => "",
                },
                _ => "",
            };

            if let Err(PtyActionError::WriteFailed(e)) = self.pty.write(text) {
                event!(Level::DEBUG, "Write Error: {}", e);
            }
        }
    }
}

impl eframe::App for RtGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.pty.read() {
            Ok(_) => (),
            Err(PtyActionError::NoContent) => (),
            Err(PtyActionError::ReadFailed(e)) => eprintln!("Error: {}", e),
            _ => (),
        };
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(self.pty.display_buf());
            ui.input(|i| self.handle_input(&i));

            let cursor = egui::Shape::Rect(egui::epaint::RectShape {
                rect: egui::Rect::EVERYTHING,
                rounding: egui::Rounding::ZERO,
                fill: egui::Color32::BLUE,
                stroke: egui::Stroke::NONE,
                fill_texture_id: egui::TextureId::default(),
                uv: egui::Rect::ZERO,
            });
            //            ui.painter().add(cursor);
        });
    }
}
