use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyInfo {
    Version(String),
    Details(DependencyDetails),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyDetails {
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexoraConfig {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    #[serde(default)]
    pub dependencies: HashMap<String, DependencyInfo>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, DependencyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
}

impl NexoraConfig {
    pub fn new(name: &str) -> Self {
        let mut scripts = HashMap::new();
        scripts.insert("dev".to_string(), "nexora run index.nx".to_string());
        scripts.insert("test".to_string(), "nexora run test.nx".to_string());
        scripts.insert("build".to_string(), "nexora run build.nx".to_string());

        Self {
            name: name.to_string(),
            version: "1.0.0".to_string(),
            description: None,
            main: Some("index.nx".to_string()),
            author: None,
            license: Some("MIT".to_string()),
            scripts,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            repository: None,
            keywords: None,
            private: None,
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        let config: Self = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;

        Ok(config)
    }

    #[allow(dead_code)]
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct NexoraGlobalConfig {
    #[serde(default)]
    pub registry: String,
    #[serde(default)]
    pub auth_token: Option<String>,
    #[serde(default)]
    pub cache_dir: Option<String>,
}

impl Default for NexoraGlobalConfig {
    fn default() -> Self {
        Self {
            registry: "https://registry.nexora.dev".to_string(),
            auth_token: None,
            cache_dir: None,
        }
    }
}

#[allow(dead_code)]
impl NexoraGlobalConfig {
    pub fn load() -> Result<Self> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        let config_path = home.join(".nexora").join("config.json");

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Self = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
        let nexora_dir = home.join(".nexora");
        fs::create_dir_all(&nexora_dir)?;

        let config_path = nexora_dir.join("config.json");
        let json = serde_json::to_string_pretty(self)?;
        fs::write(config_path, json)?;
        Ok(())
    }
}