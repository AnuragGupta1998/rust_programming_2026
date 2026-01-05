fn main() {
    let mut s = String::from("Anurag");

    //mutable borrows and references
    let s1 = &mut s;
    println!("s1- {}", s1); // Anurag

    let s2:&mut String = &mut s;
    println!("s2- {}", s2); // Anurag
    s2.push_str(" gupta");
    println!("s2 -- {}", s2); // Anurag gupta
    println!("s after s2 modification {}", s); // Anurag gupta

    //immutable borrows and references
    let s3 = &s;
    println!("s3 {}", s3); // Anurag gupta

    let s4 = &s;
    println!("s4 {}", s4);
    println!("s3 {},s4 {}", s3,s4); // Anurag gupta  Anurag gupta

    println!("s {}", s);
    //println!("{}", s2); //we can't use s2 here because its mutable borrow ended when we created s3 and s4

    append_str(&mut s);
    println!("s after append_str {}", s); // Anurag gupta is Btech Graduate engineer

    //mutable borrowing and references 
    let s5 = &mut s;
    println!("s5 {}", s5); // Anurag gupta is Btech Graduate engineer

    let s6 =  &mut s;
    //println!("s5 {}, s6 {}", s5,  s6); //we cant use both mutable references at the same time as it mention in the borrowing rules
    println!("s6 {}", s6);// Anurag gupta is Btech Graduate engineer

    let s7 =&mut s;
    println!("s7 {}", s7); // Anurag gupta is Btech Graduate engineer
    // println!("s6 {}", s6);// we cant use s6 here because s6's immutable borrow ended when we created s7

    let mut s8 = &mut s;
    chnage_str(&mut s8);
    println!("s8 {}", s8); // Anurag gupta is Btech Graduate engineer from India

}

fn append_str(str : &mut String){
    str.push_str(" is Btech Graduate engineer");
}
fn chnage_str(str : &mut &mut String){
    str.push_str(" from India");
}
