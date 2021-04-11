pub const HISTORY_DIR: &str = ".rclip";
pub const HISTORY_FILE: &str = "history.rclips";

pub struct Paths {
    pub history_dir_path: String,
    pub history_file_path: String,
}

impl Paths {
    pub fn new() -> Self {
        let history_dir_path = format!("{}/{}", dirs::home_dir().unwrap().display(), HISTORY_DIR);
        std::fs::create_dir_all(&history_dir_path).unwrap();

        let history_file_path = format!("{}/{}", history_dir_path, HISTORY_FILE);
        std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&history_file_path)
            .unwrap();

        Self {
            history_dir_path,
            history_file_path,
        }
    }
}