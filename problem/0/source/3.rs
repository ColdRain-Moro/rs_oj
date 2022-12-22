fn main() { let mut a = String::new();let mut b = String::new();std::io::stdin().read_line(&mut a).expect("");std::io::stdin().read_line(&mut b).expect("");a = a.replace("
", "").trim().to_string();b = b.replace("
", "").trim().to_string();let a: i32 = a.parse().unwrap();let b: i32 = b.parse().unwrap();println!("{}", a + b);  }