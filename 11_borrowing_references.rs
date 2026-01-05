fn main() {
    let mut s = String::from("Anurag");

    //mutable borrows and references
    let s1 = &mut s;
    println!("s1- {}", s1);

    let s2:&mut String = &mut s;
    println!("s2- {}", s2);
    s2.push_str(" gupta");
    println!("s2 -- {}", s2);
    println!("s after s2 modification {}", s);

    //immutable borrows and references
    let s3 = &s;
    println!("s3 {}", s3);

    let s4 = &s;
    println!("s4 {}", s4);
    println!("s3 {},s4 {}", s3,s4);

    println!("s {}", s);
    // println!("{}", s2); //we cant use s2 here because its mutable borrow ended when we created s3 and s4

    append_str(&mut s);
    println!("s after append_str {}", s);

    //mutable borrowing and references 
    let s5 = &mut s;
    let s6 = &mut s;
    // println!("s5 {}, s6 {}", s5, s6); //we cant use both mutable references at the same time as it mention in the borrowing rules
    println!("s5 {}", s6);
}

fn append_str(str : &mut String){
    str.push_str(" is Btech Graduate engineer");
}
