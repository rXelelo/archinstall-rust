pub fn rootget() -> i32 {
  let mut rootpsws = String::new();
  std::io::stdin()
    .read_line(&mut rootpsws)
    .expect("Failed to read line");
  let rootpsw: i32 = rootpsws.trim().parse().unwrap();
  return rootpsw;
}
