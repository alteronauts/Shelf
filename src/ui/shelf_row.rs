use egui::{
    Align, CornerRadius, FontId, Layout, Pos2, Rect, Response, RichText, Sense,
    StrokeKind, Vec2,
};

use crate::model::{Book, Shelf};

const CARD_WIDTH: f32 = 130.0;
const CARD_HEIGHT: f32 = 160.0;
const CARD_GAP: f32 = 12.0;
const CARD_ROUNDING: u8 = 6;

/// Renders a single shelf row with horizontal scrolling.
/// Returns Some(book_index) if a book was clicked.
pub fn show(ui: &mut egui::Ui, shelf: &Shelf, shelf_idx: usize) -> Option<usize> {
    let mut clicked: Option<usize> = None;

    ui.add_space(6.0);

    ui.label(
        RichText::new(&shelf.name)
            .font(FontId::proportional(15.0))
            .strong(),
    );

    ui.add_space(4.0);

    let row_height = CARD_HEIGHT + 8.0;

    ui.allocate_ui_with_layout(
        Vec2::new(ui.available_width(), row_height),
        Layout::left_to_right(Align::Center),
        |ui| {
            egui::ScrollArea::horizontal()
                .id_salt(format!("shelf_{shelf_idx}"))
                .show(ui, |ui| {
                    ui.set_min_height(row_height);

                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        ui.set_min_height(row_height);
                        ui.spacing_mut().item_spacing.x = CARD_GAP;

                        for (i, book) in shelf.books.iter().enumerate() {
                            if book_card(ui, book).clicked() {
                                clicked = Some(i);
                            }
                        }
                    });
                });
        },
    );

    ui.add_space(6.0);
    ui.separator();

    clicked
}

fn book_card(ui: &mut egui::Ui, book: &Book) -> Response {
    let (rect, response) = ui.allocate_exact_size(Vec2::new(CARD_WIDTH, CARD_HEIGHT), Sense::click());

    if ui.is_rect_visible(rect) {
        let visuals = ui.visuals();
        let is_hovered = response.hovered();

        let bg = if is_hovered {
            visuals.widgets.hovered.bg_fill
        } else {
            visuals.widgets.inactive.bg_fill
        };

        let stroke = if is_hovered {
            visuals.widgets.hovered.bg_stroke
        } else {
            visuals.widgets.inactive.bg_stroke
        };

        ui.painter()
            .rect_filled(rect, CornerRadius::same(CARD_ROUNDING), bg);
        ui.painter()
            .rect_stroke(rect, CornerRadius::same(CARD_ROUNDING), stroke, StrokeKind::Outside);

        // "Spine" accent line on the left
        let spine_rect = Rect::from_min_size(
            rect.min,
            Vec2::new(4.0, rect.height()),
        );
        let accent = if is_hovered {
            visuals.selection.bg_fill
        } else {
            visuals.widgets.active.bg_fill
        };
        ui.painter().rect_filled(
            spine_rect,
            CornerRadius {
                nw: CARD_ROUNDING,
                sw: CARD_ROUNDING,
                ..CornerRadius::ZERO
            },
            accent,
        );

        // Title text
        let text_rect = rect.shrink2(Vec2::new(14.0, 12.0));
        let text_color = if is_hovered {
            visuals.strong_text_color()
        } else {
            visuals.text_color()
        };

        let galley = ui.painter().layout(
            book.title.clone(),
            FontId::proportional(13.0),
            text_color,
            text_rect.width(),
        );

        let galley_size = galley.size();
        let text_pos = Pos2::new(
            text_rect.center().x - galley_size.x / 2.0,
            text_rect.center().y - galley_size.y / 2.0,
        );
        ui.painter().galley(text_pos, galley, text_color);
    }

    response
}
