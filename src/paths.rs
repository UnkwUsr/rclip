use crate::config::Config;

const HISTORY_DIR: &str = ".rclip";
const CONFIG_DIR: &str = "rclip";
const CONFIG_FILE: &str = "config.toml";

pub struct Paths {
    // TODO: remove "_path" at the end of fields names
    pub history_dir_path: String,
    pub config_path: String,
}

impl Paths {
    pub fn new() -> Self {
        let history_dir_path = format!("{}/{}", dirs::home_dir().unwrap().display(), HISTORY_DIR);
        std::fs::create_dir_all(&history_dir_path).unwrap();

        let config_dir_path = format!("{}/{}", dirs::config_dir().unwrap().display(), CONFIG_DIR);
        std::fs::create_dir_all(&config_dir_path).unwrap();
        let config_path = format!("{}/{}", config_dir_path, CONFIG_FILE);

        Self {
            history_dir_path,
            config_path,
        }
    }

    pub fn create_targets_dirs(&self, config: &Config) {
        for target in &config.targets_list {
            let path = format!("{}/{}", self.history_dir_path, target);
            std::fs::create_dir_all(&path).unwrap();
        }
    }
}
