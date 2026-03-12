use egui::{Align, FontId, Layout, Margin, RichText, Vec2};
use egui_commonmark::CommonMarkCache;

use crate::export;

const PADDING: i8 = 24;

/// Renders the markdown reader view.
/// Returns true if the user clicked "Back".
pub fn show(
    ui: &mut egui::Ui,
    title: &str,
    content: &str,
    cache: &mut CommonMarkCache,
) -> bool {
    let mut go_back = false;

    egui::Frame::NONE
        .inner_margin(Margin::symmetric(PADDING, PADDING))
        .show(ui, |ui| {
            // Top bar
            ui.horizontal(|ui| {
                let btn = egui::Button::new(
                    RichText::new("<  Back").font(FontId::proportional(14.0)),
                )
                .min_size(Vec2::new(80.0, 32.0))
                .corner_radius(6.0);

                if ui.add(btn).clicked() {
                    go_back = true;
                }

                ui.add_space(16.0);

                ui.label(
                    RichText::new(title)
                        .font(FontId::proportional(18.0))
                        .strong(),
                );

                // Export PDF button on the right
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let pdf_btn = egui::Button::new(
                        RichText::new("Export PDF").font(FontId::proportional(13.0)),
                    )
                    .min_size(Vec2::new(100.0, 32.0))
                    .corner_radius(6.0);

                    if ui.add(pdf_btn).clicked() {
                        let default_name = format!("{title}.pdf");
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name(&default_name)
                            .add_filter("PDF", &["pdf"])
                            .save_file()
                        {
                            if let Err(e) = export::export_to_pdf(title, content, &path) {
                                tracing::error!("PDF export failed: {e}");
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);

            // Markdown content - fill the remaining viewport
            let available = ui.available_size();
            egui::ScrollArea::vertical()
                .min_scrolled_height(available.y)
                .show(ui, |ui| {
                    ui.set_min_size(available);
                    egui_commonmark::CommonMarkViewer::new()
                        .max_image_width(Some((available.x * 0.9) as usize))
                        .show(ui, cache, content);
                });
        });

    go_back
}
