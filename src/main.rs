fn main() {

    let string1 = String::from("Sari");
    let string2 = String::from("Kirmizi");
    
    
    let concatenated_strings = concatenate_strings(&string1,&string2);
    println!("Yeni kelime: {}", concatenated_strings);
  
  }
  
  fn concatenate_strings(s1: &str, s2: &str) -> String
  {
  
      let mut result = String::new();
  
      result.push_str(s1);
      result.push_str(s2);
  
      return result;
  }