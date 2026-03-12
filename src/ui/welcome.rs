use std::path::PathBuf;

use egui::{Align, Color32, FontId, Layout, RichText, Vec2};

/// Renders the welcome screen. Returns Some(path) if the user picks a folder.
pub fn show(ui: &mut egui::Ui) -> Option<PathBuf> {
    let mut picked = None;

    let available = ui.available_size();

    ui.allocate_ui_with_layout(
        available,
        Layout::top_down(Align::Center),
        |ui| {
            ui.add_space(available.y * 0.25);

            ui.label(
                RichText::new("Shelf")
                    .font(FontId::proportional(48.0))
                    .color(ui.visuals().strong_text_color()),
            );

            ui.add_space(8.0);

            ui.label(
                RichText::new("Your markdown library")
                    .font(FontId::proportional(16.0))
                    .color(Color32::from_gray(140)),
            );

            ui.add_space(32.0);

            let btn = egui::Button::new(
                RichText::new("Open Folder").font(FontId::proportional(15.0)),
            )
            .min_size(Vec2::new(160.0, 44.0))
            .corner_radius(8.0);

            if ui.add(btn).clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    picked = Some(path);
                }
            }
        },
    );

    picked
}
