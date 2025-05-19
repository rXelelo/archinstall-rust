use std::process::Command;

//install arch
pub fn install_arch(loader: String, rootpsw: i32, usrname: String, usrpsw: i32, timezone: String) {
  // test 
  println!("Loader: {loader}\nRootPSW: {rootpsw}\nUserName: {usrname}\nUserpsw: {usrpsw}\nTimeZone: {timezone}");
  let _ = Command::new("pacstrap").args(["-K", "/mnt", "base", "linux", "linux-firmware"]).status(); 

}
