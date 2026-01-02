fn main(){
    let s1:String = String::from("hello");
    let length = calculate_length(s1.clone());
    println!("The length of '{}' is {}.", s1, length);
}

fn calculate_length(s: String) -> usize {
    s.len()
}