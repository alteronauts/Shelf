use std::path::PathBuf;

fn config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("shelf"))
}

fn ignore_file() -> Option<PathBuf> {
    config_dir().map(|d| d.join("ignored.txt"))
}

pub fn load_ignore_text() -> String {
    ignore_file()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .unwrap_or_default()
}

pub fn save_ignore_text(text: &str) {
    if let Some(dir) = config_dir() {
        let _ = std::fs::create_dir_all(&dir);
        if let Some(path) = ignore_file() {
            if let Err(e) = std::fs::write(&path, text) {
                tracing::error!("Failed to save ignore list: {e}");
            }
        }
    }
}
