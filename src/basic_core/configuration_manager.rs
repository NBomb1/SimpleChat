use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppConfig {
    pub username: String,
    pub ip: String,
    pub port: String,
}

pub struct ConfigurationManager {
    pub config: AppConfig,
    file_path: String,
}

impl ConfigurationManager {
    pub fn new() -> Self {
        let file_path = "config.json".to_string();
        let default_config = AppConfig {
            username: "".to_string(),
            ip: "".to_string(),
            port: "".to_string(),
        };

        let mut manager = Self {
            config: default_config,
            file_path,
        };

        manager.load();
        log::info!("Configuration manager loaded.");
        log::info!("Read: \n\t\
        Username: {0}; \n\t\
        Ip:{1}; \n\t\
        Port: {2}", manager.config.username, manager.config.ip, manager.config.port);
        manager
    }

    pub fn load(&mut self) {
        if Path::new(&self.file_path).exists() {
            if let Ok(mut file) = File::open(&self.file_path) {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(parsed) = serde_json::from_str::<AppConfig>(&contents) {
                        self.config = parsed;
                        log::info!("Config loaded successfully");
                    }
                }
            }
        }
    }

    pub fn save(&mut self, username: String, ip: String, port: String) {
        self.config.username = username;
        self.config.ip = ip;
        self.config.port = port;

        if let Ok(pushed_json) = serde_json::to_string_pretty(&self.config) {
            if let Ok(mut file) = File::create(&self.file_path) {
                let _ = file.write_all(pushed_json.as_bytes());
                log::info!("Config has been saved!");
            }
        }
    }
}
