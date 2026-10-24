use std::path::{Path, PathBuf};

//  This constant defines the max numbers of recents you can have
const MAX_RECENT: usize = 6;

fn recent_file() -> PathBuf {
    gtk4::glib::user_data_dir().join("shelfies").join("recent.txt")
}

//  Existing recent paths only, most recent first
pub fn load() -> Vec<String> {
    std::fs::read_to_string(recent_file())
        .unwrap_or_default()
        .lines()
        .filter(|p| Path::new(p).exists())
        .map(String::from)
        .collect()
}

pub fn add(path: &str) {
    let mut recent = load();
    recent.retain(|p| p != path);
    recent.insert(0, path.to_string());
    recent.truncate(MAX_RECENT);

    let file = recent_file();
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(file, recent.join("\n"));
}