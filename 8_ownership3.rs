fn main(){
    let s1:String = String::from("hello");
    let length = calculate_length(s1.clone()); // it copy the s1 to the function calculate_length s argument
    println!("The length of '{}' is {}.", s1, length); //it work because we are not dealing with ownership concept of rust here
}

fn calculate_length(s: String) -> usize {
    s.len() // return the length of the string s
}