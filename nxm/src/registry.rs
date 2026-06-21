use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::config::NexoraConfig;
use crate::package::Package;

pub struct RegistryClient {
    client: Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RegistryResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub downloads: u64,
    pub stars: u64,
}

impl RegistryClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("nxm/1.0.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: "http://localhost:3000".to_string(),
        }
    }

    pub fn get_package(&self, name: &str) -> Result<Package> {
        let url = format!("{}/api/packages/{}", self.base_url, name);

        let response = self
            .client
            .get(&url)
            .send()
            .with_context(|| format!("Failed to fetch package '{}'", name))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Package '{}' not found in registry",
                name
            ));
        }

        let body = response
            .text()
            .with_context(|| format!("Failed to read response for '{}'", name))?;

        let data: serde_json::Value = serde_json::from_str(&body)
            .with_context(|| format!("Failed to parse response for '{}'", name))?;

        let version = data.get("version")
            .and_then(|v| v.as_str())
            .unwrap_or("1.0.0")
            .to_string();

        let description = data.get("description")
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();

        let author = data.get("author")
            .and_then(|a| a.as_str())
            .map(|s| s.to_string());

        let package = Package {
            name: name.to_string(),
            description: Some(description),
            author,
            license: Some("MIT".to_string()),
            repository: None,
            keywords: None,
            dist_tags: crate::package::DistTags {
                latest: version.clone(),
                next: None,
            },
            versions: {
                let mut versions = std::collections::HashMap::new();
                versions.insert(version.clone(), crate::package::VersionInfo {
                    version: version.clone(),
                    dist: crate::package::DistInfo {
                        tarball: format!("{}/api/packages/{}/download", self.base_url, name),
                        shasum: None,
                        integrity: None,
                    },
                    dependencies: None,
                    dev_dependencies: None,
                    scripts: None,
                });
                versions
            },
            dependencies: std::collections::HashMap::new(),
            dev_dependencies: std::collections::HashMap::new(),
        };

        Ok(package)
    }

    pub fn search_packages(&self, query: &str) -> Result<Vec<SearchResult>> {
        let url = format!("{}/api/search?q={}", self.base_url, query);

        let response = self
            .client
            .get(&url)
            .send()
            .with_context(|| "Failed to search packages")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("Search failed"));
        }

        let results: Vec<SearchResult> = response
            .json()
            .with_context(|| "Failed to parse search results")?;

        Ok(results)
    }

    pub fn download_package(
        &self,
        name: &str,
        _version: &str,
        dest: &Path,
    ) -> Result<()> {
        let url = format!(
            "{}/api/packages/{}/download",
            self.base_url, name
        );

        let response = self
            .client
            .get(&url)
            .send()
            .with_context(|| format!("Failed to download package '{}'", name))?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to download package '{}'",
                name
            ));
        }

        let body = response
            .text()
            .with_context(|| "Failed to read response")?;

        let data: serde_json::Value = serde_json::from_str(&body)
            .with_context(|| "Failed to parse response")?;

        if let Some(files) = data.get("files").and_then(|f| f.as_array()) {
            for file in files {
                if let (Some(path), Some(content)) = (file.get("path"), file.get("content")) {
                    let file_path = dest.join(path.as_str().unwrap_or(""));
                    if let Some(parent) = file_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    fs::write(&file_path, content.as_str().unwrap_or(""))?;
                }
            }
        }

        Ok(())
    }

    pub fn publish_package(
        &self,
        config: &NexoraConfig,
        _tarball_path: &Path,
        access: &str,
    ) -> Result<()> {
        let url = format!("{}/api/packages", self.base_url);

        let body = serde_json::json!({
            "name": config.name,
            "version": config.version,
            "description": config.description,
            "access": access,
        });

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .with_context(|| "Failed to publish package")?;

        if !response.status().is_success() {
            let error_text = response.text().unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Failed to publish package: {}",
                error_text
            ));
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_auth_token(&self) -> Result<Option<String>> {
        let config = crate::config::NexoraGlobalConfig::load()?;
        Ok(config.auth_token)
    }

    #[allow(dead_code)]
    pub fn set_auth_token(&self, token: &str) -> Result<()> {
        let mut config = crate::config::NexoraGlobalConfig::load()?;
        config.auth_token = Some(token.to_string());
        config.save()
    }
}