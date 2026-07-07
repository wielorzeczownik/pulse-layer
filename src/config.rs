use std::{fs, path::PathBuf};

use crate::settings::AppSettings;

fn path() -> Option<PathBuf> {
  let mut dir = dirs::config_dir()?;
  dir.push("pulselayer");
  dir.push("settings.json");
  Some(dir)
}

/// Load saved settings, falling back to defaults if the file is missing or corrupt.
pub fn load() -> AppSettings {
  let Some(file) = path() else {
    return AppSettings::default();
  };
  let Ok(data) = fs::read_to_string(&file) else {
    return AppSettings::default();
  };
  serde_json::from_str(&data).unwrap_or_default()
}

/// Persist settings to disk. Silently ignores I/O errors.
pub fn save(settings: &AppSettings) {
  let Some(file) = path() else {
    return;
  };
  if let Some(parent) = file.parent() {
    let _ = fs::create_dir_all(parent);
  }
  if let Ok(json) = serde_json::to_string_pretty(settings) {
    let _ = fs::write(file, json);
  }
}
