use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub versions: HashMap<String, VersionInfo>,
    pub dist_tags: DistTags,
    #[serde(default)]
    pub dependencies: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub dist: DistInfo,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub scripts: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistInfo {
    pub tarball: String,
    pub shasum: Option<String>,
    pub integrity: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistTags {
    pub latest: String,
    #[serde(rename = "next")]
    pub next: Option<String>,
}

impl Package {
    pub fn get_latest_version(&self) -> String {
        self.dist_tags.latest.clone()
    }

    #[allow(dead_code)]
    pub fn get_version(&self, version: &str) -> Option<&VersionInfo> {
        self.versions.get(version)
    }

    #[allow(dead_code)]
    pub fn get_all_versions(&self) -> Vec<&str> {
        self.versions.keys().map(|s| s.as_str()).collect()
    }

    #[allow(dead_code)]
    pub fn get_latest_tarball_url(&self) -> Option<String> {
        let version = self.get_latest_version();
        self.versions
            .get(&version)
            .map(|v| v.dist.tarball.clone())
    }

    #[allow(dead_code)]
    pub fn has_dependency(&self, name: &str, version: &str) -> bool {
        if let Some(deps) = self.dependencies.get(version) {
            deps.contains_key(name)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub scripts: HashMap<String, String>,
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}

#[allow(dead_code)]
impl PackageManifest {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: None,
            main: None,
            author: None,
            license: None,
            scripts: HashMap::new(),
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub downloads: u64,
    pub stars: u64,
    pub created_at: String,
    pub updated_at: String,
}