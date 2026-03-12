use egui::{Align, FontId, Layout, RichText, Vec2};

use crate::model::Library;
use crate::scanner;
use crate::ui::shelf_row;

pub enum LibraryAction {
    None,
    OpenFolder,
    OpenBook { shelf_idx: usize, book_idx: usize },
    Rescan,
}

pub fn show(
    ui: &mut egui::Ui,
    library: &Library,
    ignore_text: &mut String,
    show_ignore_panel: &mut bool,
) -> LibraryAction {
    let mut action = LibraryAction::None;

    // Toolbar
    ui.horizontal(|ui| {
        let btn = egui::Button::new(
            RichText::new("Open Folder").font(FontId::proportional(13.0)),
        )
        .min_size(Vec2::new(110.0, 32.0))
        .corner_radius(6.0);

        if ui.add(btn).clicked() {
            action = LibraryAction::OpenFolder;
        }

        ui.add_space(8.0);

        ui.label(
            RichText::new(library.root.to_string_lossy().as_ref())
                .font(FontId::proportional(12.0))
                .weak(),
        );

        // Right side: Ignored folders button
        ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
            let label = if *show_ignore_panel { "Hide Ignored" } else { "Ignored" };
            let btn = egui::Button::new(
                RichText::new(label).font(FontId::proportional(12.0)),
            )
            .min_size(Vec2::new(90.0, 28.0))
            .corner_radius(6.0);

            if ui.add(btn).clicked() {
                *show_ignore_panel = !*show_ignore_panel;
            }
        });
    });

    ui.separator();

    // Ignore panel
    if *show_ignore_panel {
        egui::Frame::NONE
            .inner_margin(egui::Margin::symmetric(8, 8))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("Ignored folders (one per line):")
                        .font(FontId::proportional(12.0))
                        .weak(),
                );

                ui.add_space(4.0);

                let builtin = scanner::BUILTIN_IGNORE.join(", ");
                ui.label(
                    RichText::new(format!("Built-in: {builtin}"))
                        .font(FontId::proportional(10.0))
                        .weak(),
                );

                ui.add_space(4.0);

                let response = ui.add(
                    egui::TextEdit::multiline(ignore_text)
                        .desired_rows(4)
                        .desired_width(ui.available_width())
                        .hint_text("e.g. drafts\nprivate\narchive")
                        .font(FontId::monospace(12.0)),
                );

                ui.add_space(4.0);

                let save_btn = egui::Button::new(
                    RichText::new("Apply").font(FontId::proportional(13.0)),
                )
                .min_size(Vec2::new(80.0, 28.0))
                .corner_radius(6.0);

                if ui.add(save_btn).clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command)) {
                    action = LibraryAction::Rescan;
                }
            });

        ui.separator();
    }

    // Shelves
    if library.shelves.is_empty() {
        ui.add_space(40.0);
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("No markdown files found in this folder.")
                    .font(FontId::proportional(16.0))
                    .weak(),
            );
        });
    } else {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (shelf_idx, shelf) in library.shelves.iter().enumerate() {
                if let Some(book_idx) = shelf_row::show(ui, shelf, shelf_idx) {
                    action = LibraryAction::OpenBook {
                        shelf_idx,
                        book_idx,
                    };
                }
            }
            ui.add_space(20.0);
        });
    }

    action
}
