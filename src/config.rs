use std::{fs, io, path::Path};

use serde::Deserialize;

// Struct to hold the localized strings loaded from a JSON file.
// The field names must match the keys in the JSON files.
#[derive(Deserialize)]
pub struct LocConfigStrings {
  pub keyboard_layout_cfg: String,
  pub locale_language_cfg: String,
  pub locale_encoding_cfg: String
}

// Function to load localization strings from a JSON file based on the language code.
// It constructs the file path relative to the project's manifest directory.
pub fn load_localConfigStrings() -> Result<LocConfigStringsStrings, io::Error> {
  let manifest_dir = env!("CARGO_MANIFEST_DIR");
  let file_path = Path::new(manifest_dir)
    .join("config")
    .join(format!("{}.json"));
  let file_content = fs::read_to_string(file_path)?;
  let localized_strings: LocConfigStrings = serde_json::from_str(&file_content)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
  Ok(localized_strings)
}
