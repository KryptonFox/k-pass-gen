use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

const DIR_NAME: &str = "KPassGen";
const FILE_NAME: &str = "config.toml";

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub len: usize,
    pub letters: Letters,
    pub numbers: Numbers,
    pub special_chars: SpecialChars,
}

impl Config {
    pub fn new() -> Self {
        ConfigFile::load().unwrap_or_default().config
    }

    pub fn save(&self) {
        let _ = ConfigFile::from(self).save();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            len: 8,
            letters: Letters {
                enabled: true,
                use_capitals: true,
                chars: "abcdefghijklmnopqrstuvwxyz".to_string()
            },
            numbers: Numbers {
                enabled: true,
                chars: "0123456789".to_string()
            },
            special_chars: SpecialChars {
                enabled: true,
                chars: "([{?*&%$#@}])".to_string()
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Letters {
    pub enabled: bool,
    pub use_capitals: bool,
    pub chars: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Numbers {
    pub enabled: bool,
    pub chars: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpecialChars {
    pub enabled: bool,
    pub chars: String,
}

#[derive(Serialize, Deserialize, Default)]
struct ConfigFile {
    pub config: Config,
}

impl From<&Config> for ConfigFile {
    fn from(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

impl ConfigFile {
    fn cfg_dir() -> std::io::Result<std::path::PathBuf> {
        let cfg_dir = dirs::config_dir().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Cannot get config directory",
        ))?;
        Ok(cfg_dir.join(DIR_NAME))
    }
    pub fn load() -> std::io::Result<Self> {
        let cfg = ConfigFile::default();
        let path = Self::cfg_dir()?.join(FILE_NAME);

        if !path.exists() {
            cfg.save()?;
            Ok(cfg)
        } else {
            let mut file = File::open(&path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let cfg = toml::from_str::<Self>(contents.as_str()).map_err(|e| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, format!("{}", e))
            })?;
            Ok(cfg)
        }
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::cfg_dir()?;

        if !path.exists() {
            fs::create_dir(path.clone())?;
        }

        let mut file = File::create(path.join(FILE_NAME))?;
        file.write_all(toml::to_string_pretty(self).unwrap().as_bytes())?;

        Ok(())
    }
}
