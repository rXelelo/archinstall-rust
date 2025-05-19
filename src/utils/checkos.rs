use std::{env, path::Path, process};

pub fn check() {
  let current_os = env::consts::OS;

  if current_os == "linux" {
    let is_arch_linux = Path::new("/etc/arch-release").exists();
    if is_arch_linux {
      println!("Running on Arch Linux. Proceeding.");
    } else {
      println!("Running on Linux. Proceeding.");
    }
  } else {
    let os_display_name = match current_os {
      "macos" => "macOS",
      "windows" => "Windows",
      _ => current_os, // For other OSes, use their system name
    };

    if !cfg!(debug_assertions) {
      eprintln!(
        "ERROR: This program is intended to run only on Linux. Detected OS: {}.",
        os_display_name
      );
      process::exit(1);
    } else {
      // In Debug mode, continue, but issue a warning.
      eprintln!(
        "WARNING: Not running on Linux. Detected OS: {}. Proceeding because running in debug mode.",
        os_display_name
      );
    }
  }
}
