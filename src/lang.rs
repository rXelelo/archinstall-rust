use std::{fs, io, path::Path};

use serde::Deserialize;

// Struct to hold the localized strings loaded from a JSON file.
// The field names must match the keys in the JSON files.
#[derive(Deserialize)]
pub struct LocalizationStrings {
  pub language_menu_option: String,
  pub exit_option: String,
  pub setup_header: String,
  pub disk_config: String,
  pub bootloader_select: String,
  pub root_password: String,
  pub setup_user: String,
  pub install_archlinux: String,
  pub select_language_header: String,
  pub disk_management: String,
  pub mounting: String,
  pub help_menu1: String,
  pub help_menu2: String,
  pub help_menu3: String,
}

// Function to load localization strings from a JSON file based on the language code.
// It constructs the file path relative to the project's manifest directory.
pub fn load_localization(lang_code: &str) -> Result<LocalizationStrings, io::Error> {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  let file_path = Path::new(manifest_dir)
    .join("lang")
    .join(format!("{}.json", lang_code));
  let file_content = fs::read_to_string(file_path)?;
  let localized_strings: LocalizationStrings = serde_json::from_str(&file_content)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
  Ok(localized_strings)
}
