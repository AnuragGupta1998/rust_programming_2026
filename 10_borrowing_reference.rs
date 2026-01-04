fn main() {
 println!("Borrowing and reference example");

 let str1 = String::from("Anurag");
 let s1 = &str1; // passing reference and s1 now borrowing Anurag from str1

 println!("{} {}",str1,s1); // Anurag  Anurag

 //mutable reference example
 let mut str2 = String::from("Hello");
 let s2 = &str2;
 println!(" {} {}",str2,s2); // Hello Hello

 //mutable reference example
 let s3 = &mut str2; // mutable reference
 s3.push_str(", world!");
 print!(" {}",s3); // Hello, world!
 let s4 = &mut str2;
 s4.push_str(" Welcome to Rust programming.");   
 println!(" {}",s4); // Hello, world! Welcome to Rust programming.
}

