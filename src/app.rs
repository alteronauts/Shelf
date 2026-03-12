use egui_commonmark::CommonMarkCache;

use crate::model::{Library, Screen};
use crate::persist;
use crate::scanner;
use crate::ui::{library, reader, welcome};

pub struct App {
    screen: Screen,
    library: Option<Library>,
    md_cache: CommonMarkCache,
    current_md_content: Option<String>,
    ignore_text: String,
    show_ignore_panel: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Welcome,
            library: None,
            md_cache: CommonMarkCache::default(),
            current_md_content: None,
            ignore_text: persist::load_ignore_text(),
            show_ignore_panel: false,
        }
    }
}

impl App {
    fn user_ignore_list(&self) -> Vec<String> {
        self.ignore_text
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect()
    }

    fn rescan(&mut self) {
        if let Some(ref lib) = self.library {
            let root = lib.root.clone();
            self.library = Some(scanner::scan_folder(&root, &self.user_ignore_list()));
        }
    }

    fn open_folder(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            let lib = scanner::scan_folder(&path, &self.user_ignore_list());
            self.library = Some(lib);
            self.screen = Screen::Browse;
            self.current_md_content = None;
        }
    }

    fn open_book(&mut self, shelf_idx: usize, book_idx: usize) {
        if let Some(ref lib) = self.library {
            if let Some(shelf) = lib.shelves.get(shelf_idx) {
                if let Some(book) = shelf.books.get(book_idx) {
                    match std::fs::read_to_string(&book.path) {
                        Ok(content) => {
                            self.current_md_content = Some(content);
                            self.screen = Screen::Reading {
                                shelf_idx,
                                book_idx,
                            };
                        }
                        Err(e) => {
                            tracing::error!("Failed to read {}: {e}", book.path.display());
                        }
                    }
                }
            }
        }
    }

    fn go_back(&mut self) {
        self.screen = Screen::Browse;
        self.current_md_content = None;
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing = egui::vec2(8.0, 6.0);

            match self.screen {
                Screen::Welcome => {
                    if let Some(path) = welcome::show(ui) {
                        let lib = scanner::scan_folder(&path, &self.user_ignore_list());
                        self.library = Some(lib);
                        self.screen = Screen::Browse;
                    }
                }
                Screen::Browse => {
                    // Clone library ref data needed for display to avoid borrow conflicts
                    if self.library.is_some() {
                        let lib = self.library.as_ref().unwrap();
                        match library::show(
                            ui,
                            lib,
                            &mut self.ignore_text,
                            &mut self.show_ignore_panel,
                        ) {
                            library::LibraryAction::None => {}
                            library::LibraryAction::OpenFolder => {
                                self.open_folder();
                            }
                            library::LibraryAction::OpenBook {
                                shelf_idx,
                                book_idx,
                            } => {
                                self.open_book(shelf_idx, book_idx);
                            }
                            library::LibraryAction::Rescan => {
                                persist::save_ignore_text(&self.ignore_text);
                                self.rescan();
                            }
                        }
                    }
                }
                Screen::Reading {
                    shelf_idx,
                    book_idx,
                } => {
                    let title = self
                        .library
                        .as_ref()
                        .and_then(|lib| lib.shelves.get(shelf_idx))
                        .and_then(|s| s.books.get(book_idx))
                        .map(|b| b.title.as_str())
                        .unwrap_or("Unknown");

                    let content = self
                        .current_md_content
                        .as_deref()
                        .unwrap_or("");

                    let title_owned = title.to_string();
                    let content_owned = content.to_string();

                    if reader::show(ui, &title_owned, &content_owned, &mut self.md_cache) {
                        self.go_back();
                    }
                }
            }
        });
    }
}
