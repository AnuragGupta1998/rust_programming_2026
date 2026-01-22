//macro that means code that write more code is called metaprogramming 
//marco is a form of metaprogramming in rust
//macro_rules! is used to define macros in rust

macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();  // Expands to: println!("Hello, world!");
}