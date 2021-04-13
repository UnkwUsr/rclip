use crate::config::Config;

const HISTORY_DIR: &str = ".rclip";
const HISTORY_FILE: &str = "history.rclips";
const CONFIG_DIR: &str = "rclip";
const CONFIG_FILE: &str = "config.toml";
const OTHER_TARGETS_DIR: &str = "other_targets";

pub struct Paths {
    // TODO: remove "_path" at the end of fields names
    pub history_dir_path: String,
    pub history_file_path: String,
    pub config_path: String,
    pub other_targets_dir_path: String,
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

        let config_dir_path = format!("{}/{}", dirs::config_dir().unwrap().display(), CONFIG_DIR);
        std::fs::create_dir_all(&config_dir_path).unwrap();
        let config_path = format!("{}/{}", config_dir_path, CONFIG_FILE);

        let other_targets_dir_path = format!("{}/{}", history_dir_path, OTHER_TARGETS_DIR);
        std::fs::create_dir_all(&other_targets_dir_path).unwrap();

        Self {
            history_dir_path,
            history_file_path,
            config_path,
            other_targets_dir_path,
        }
    }

    pub fn create_other_targets_dirs(&self, config: &Config) {
        for target in &config.other_targets {
            let path = format!("{}/{}", self.other_targets_dir_path, target);
            std::fs::create_dir_all(&path).unwrap();
        }
    }
}
