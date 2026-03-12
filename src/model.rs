use std::path::PathBuf;

/// A single markdown file displayed as a "book" on a shelf.
pub struct Book {
    pub title: String,
    pub path: PathBuf,
}

/// A folder containing at least one .md file, displayed as a shelf row.
pub struct Shelf {
    pub name: String,
    pub books: Vec<Book>,
}

/// The entire scanned library from a root folder.
pub struct Library {
    pub root: PathBuf,
    pub shelves: Vec<Shelf>,
}

/// Which screen is currently displayed.
pub enum Screen {
    Welcome,
    Browse,
    Reading { shelf_idx: usize, book_idx: usize },
}

