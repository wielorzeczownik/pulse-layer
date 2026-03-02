use std::{fs, path::PathBuf};

use crate::settings::AppSettings;

fn path() -> Option<PathBuf> {
  let mut p = dirs::config_dir()?;
  p.push("pulselayer");
  p.push("settings.json");
  Some(p)
}

/// Load saved settings, falling back to defaults if the file is missing or corrupt.
pub fn load() -> AppSettings {
  let p = match path() {
    Some(p) => p,
    None => return AppSettings::default(),
  };
  let data = match fs::read_to_string(&p) {
    Ok(d) => d,
    Err(_) => return AppSettings::default(),
  };
  serde_json::from_str(&data).unwrap_or_default()
}

/// Persist settings to disk. Silently ignores I/O errors.
pub fn save(settings: &AppSettings) {
  let p = match path() {
    Some(p) => p,
    None => return,
  };
  if let Some(parent) = p.parent() {
    let _ = fs::create_dir_all(parent);
  }
  if let Ok(json) = serde_json::to_string_pretty(settings) {
    let _ = fs::write(p, json);
  }
}
