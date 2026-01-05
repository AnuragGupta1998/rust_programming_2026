fn main() {
    println!("Hello, world! Anurag in Rust and Solana Development");

    let mut s1:String = String::from ("Anurag");

    println!("String before modification: {}", s1);
    println!("capacity{} length{} pointer{:p}", s1.capacity(), s1.len(), s1.as_ptr() );
    
    s1.push_str(" Gupta");
    println!("String after modification: {}", s1);
    println!("capacity{} length{} pointer{:p}", s1.capacity(), s1.len(), s1.as_ptr() );
}