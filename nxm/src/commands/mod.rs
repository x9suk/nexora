pub mod init;
pub mod install;
pub mod publish;
pub mod search;
pub mod run;
pub mod test;
pub mod update;
pub mod list;
pub mod remove;
pub mod modules;

use anyhow::Result;

#[allow(dead_code)]
pub fn ensure_nexora_home() -> Result<std::path::PathBuf> {
    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let nexora_home = home.join(".nexora");
    std::fs::create_dir_all(&nexora_home)?;
    Ok(nexora_home)
}