use anyhow::Result;
use std::collections::{HashMap, HashSet};

use crate::config::{DependencyInfo, NexoraConfig};
use crate::package::Package;
use crate::registry::RegistryClient;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ResolvedDependency {
    pub name: String,
    pub version: String,
    pub dependencies: Vec<ResolvedDependency>,
}

pub struct DependencyResolver {
    client: RegistryClient,
    #[allow(dead_code)]
    cache: HashMap<String, Package>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            client: RegistryClient::new(),
            cache: HashMap::new(),
        }
    }

    pub fn resolve(&self, config: &NexoraConfig) -> Result<Vec<ResolvedDependency>> {
        let mut resolved = Vec::new();
        let mut visited = HashSet::new();

        // Resolve regular dependencies
        for (name, dep_info) in &config.dependencies {
            let version = match dep_info {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };

            if !visited.contains(name) {
                let dep = self.resolve_package(name, &version, &mut visited)?;
                resolved.push(dep);
            }
        }

        // Resolve dev dependencies
        for (name, dep_info) in &config.dev_dependencies {
            let version = match dep_info {
                DependencyInfo::Version(v) => v.clone(),
                DependencyInfo::Details(d) => d.version.clone(),
            };

            if !visited.contains(name) {
                let dep = self.resolve_package(name, &version, &mut visited)?;
                resolved.push(dep);
            }
        }

        Ok(resolved)
    }

    fn resolve_package(
        &self,
        name: &str,
        version: &str,
        visited: &mut HashSet<String>,
    ) -> Result<ResolvedDependency> {
        visited.insert(name.to_string());

        let package = self.get_or_fetch_package(name)?;
        let resolved_version = self.resolve_version(&package, version)?;

        let mut sub_dependencies = Vec::new();

        if let Some(deps) = package.dependencies.get(&resolved_version) {
            for (dep_name, dep_version) in deps {
                if !visited.contains(dep_name) {
                    let sub_dep = self.resolve_package(dep_name, dep_version, visited)?;
                    sub_dependencies.push(sub_dep);
                }
            }
        }

        Ok(ResolvedDependency {
            name: name.to_string(),
            version: resolved_version,
            dependencies: sub_dependencies,
        })
    }

    fn get_or_fetch_package(&self, name: &str) -> Result<Package> {
        // In a real implementation, you'd check cache first
        let package = self.client.get_package(name)?;
        Ok(package)
    }

    fn resolve_version(&self, package: &Package, version_spec: &str) -> Result<String> {
        // Simple version resolution - in real implementation, use semver
        if version_spec.starts_with('^') || version_spec.starts_with('~') {
            // Return latest version for now
            Ok(package.get_latest_version())
        } else if package.versions.contains_key(version_spec) {
            Ok(version_spec.to_string())
        } else {
            Ok(package.get_latest_version())
        }
    }

    #[allow(dead_code)]
    pub fn check_conflicts(
        &self,
        deps: &[ResolvedDependency],
    ) -> Result<Vec<String>> {
        let mut conflicts = Vec::new();
        let mut versions: HashMap<String, Vec<String>> = HashMap::new();

        self.collect_versions(deps, &mut versions);

        for (name, version_list) in &versions {
            let mut unique_versions: Vec<String> = version_list.clone();
            unique_versions.sort();
            unique_versions.dedup();

            if unique_versions.len() > 1 {
                conflicts.push(format!(
                    "Package '{}' has conflicting versions: {}",
                    name,
                    unique_versions.join(", ")
                ));
            }
        }

        Ok(conflicts)
    }

    #[allow(dead_code)]
    fn collect_versions(
        &self,
        deps: &[ResolvedDependency],
        versions: &mut HashMap<String, Vec<String>>,
    ) {
        for dep in deps {
            versions
                .entry(dep.name.clone())
                .or_insert_with(Vec::new)
                .push(dep.version.clone());

            self.collect_versions(&dep.dependencies, versions);
        }
    }
}