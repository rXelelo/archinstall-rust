use std::process::Command;
use crate::ui::menu::install;
//install arch
pub fn install_arch(loader: String, rootpsw: i32, usrname: String, usrpsw: i32, timezone: String, lang: &str) {
  // test 
  println!("Loader: {loader}\nRootPSW: {rootpsw}\nUserName: {usrname}\nUserpsw: {usrpsw}\nTimeZone: {timezone}");
  if loader == "" || rootpsw == 0 || usrname == "" || usrpsw == 0 || timezone == "" {
    install::menu_install(lang, 1);
  }
  // config dir /root/.config/archinstall-rust
  let _ = Command::new("pacstrap").args(["-K", "/mnt", "base", "linux", "linux-firmware"]).status(); 
  
}
