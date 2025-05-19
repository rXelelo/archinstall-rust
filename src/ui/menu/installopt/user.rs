pub fn usrname() -> String {
  let mut usrname = String::new();
  std::io::stdin()
    .read_line(&mut usrname)
    .expect("Failed to read line");
  return usrname;
}
pub fn usrpsw() -> i32 {
  let mut usrpsws = String::new();
  std::io::stdin()
    .read_line(&mut usrpsws)
    .expect("Failed to read line");
  let usrpsw: i32 = usrpsws.trim().parse().unwrap();
  return usrpsw;
}
