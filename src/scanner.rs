use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::model::{Book, Library, Shelf};

/// Default patterns always ignored (common dev/system folders).
pub const BUILTIN_IGNORE: &[&str] = &[
    ".git",
    ".svn",
    ".hg",
    ".claude",
    "node_modules",
    "target",
    ".build",
    "__pycache__",
    ".venv",
    "venv",
    ".DS_Store",
    ".idea",
    ".vscode",
    "dist",
    "build",
    ".next",
    ".cache",
    ".expo",
    ".turbo",
];

/// Check if an entry should be ignored.
/// - Simple names (no `/`) match against every path component (so `.claude`
///   catches `.claude/agents/foo.md` too).
/// - Path patterns (with `/`) match against the relative path prefix from root.
fn is_ignored(entry_path: &Path, root: &Path, simple: &[String], paths: &[String]) -> bool {
    if let Ok(rel) = entry_path.strip_prefix(root) {
        // Check simple name patterns against every component
        for component in rel.components() {
            let name = component.as_os_str().to_string_lossy();
            if simple.iter().any(|p| p == name.as_ref()) {
                return true;
            }
        }

        // Check path prefix patterns
        if !paths.is_empty() {
            let rel_str = rel.to_string_lossy();
            for pattern in paths {
                if rel_str == *pattern || rel_str.starts_with(&format!("{pattern}/")) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn scan_folder(root: &Path, user_ignore: &[String]) -> Library {
    // Split patterns into simple names vs path patterns
    let mut simple: Vec<String> = BUILTIN_IGNORE.iter().map(|s| s.to_string()).collect();
    let mut paths: Vec<String> = Vec::new();

    for entry in user_ignore {
        let trimmed = entry.trim().to_string();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.contains('/') {
            paths.push(trimmed);
        } else {
            simple.push(trimmed);
        }
    }

    let mut groups: BTreeMap<PathBuf, Vec<Book>> = BTreeMap::new();

    // We can't use filter_entry for path patterns (it only sees the entry, not root),
    // so we use a manual iterator approach.
    let walker = WalkDir::new(root)
        .follow_links(true)
        .into_iter();

    for entry in walker {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if is_ignored(path, root, &simple, &paths) {
            continue;
        }

        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("md") {
                    let parent = path.parent().unwrap_or(root).to_path_buf();
                    let title = path
                        .file_stem()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_else(|| "Untitled".to_string());

                    groups.entry(parent).or_default().push(Book {
                        title,
                        path: path.to_path_buf(),
                    });
                }
            }
        }
    }

    let mut shelves: Vec<Shelf> = groups
        .into_iter()
        .map(|(dir, mut books)| {
            books.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));

            let name = if dir == root {
                dir.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "Library".to_string())
            } else {
                dir.strip_prefix(root)
                    .map(|rel| rel.to_string_lossy().replace(std::path::MAIN_SEPARATOR, " / "))
                    .unwrap_or_else(|_| {
                        dir.file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "Unknown".to_string())
                    })
            };

            Shelf { name, books }
        })
        .collect();

    shelves.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    Library {
        root: root.to_path_buf(),
        shelves,
    }
}
