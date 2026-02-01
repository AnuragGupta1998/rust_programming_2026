fn main() {
    let str1 = String::from("one");
    let str2 = String::from("two");
    let result;

    {
        let str3 = String::from("three");
        result = longest_str(&str1, &str2, &str3);
        println!("{}", result);
    }
    // println!("{}", result); //becuase str3 is dropped here
    
    let result2;
    {
        let str4 = String::from("fourwww");
        result2 =  longest_str2(&str1, &str4);
        println!("{}", result2);   
    }
    // println!("{}", result2);    //because str4 is dropped here
}

fn longest_str<'a, 'b:'a>(s1: &'a String, s2: &'a String, s3: &'b String) -> &'a String {
    if s1.len() > s2.len() && s1.len() > s3.len() {
         s1
    }
    else if s2.len() > s1.len() && s2.len() > s3.len() {
        s2
    }
    else {
        s3
    }
}

fn longest_str2<'a,'b:'a>(s1: &'a String , s4: &'b String) -> &'a String{
    // if s1.len() < s4.len() {s4} else{s1}
    if s1.len()>s4.len(){
        s1
    }  
    else{
        s4
    }
}
