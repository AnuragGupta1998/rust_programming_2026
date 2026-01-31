//lifetimes
fn main(){
    let s1 = String::from("Anurag");
    let s2 = String::from ("Ashutosh");

    let result = max_length_string(&s1,&s2);
    println!("The longest string is {}", result);

    println!("the longest string is from other funtion {}",get_length(&s1,&s2));

    let s3= String::from("hemanjjjjjjt");

    println!("the longest string is from other funtion2 {}",get_length2(&s1,&s2,&s3));
}

fn max_length_string<'a >(n1: &'a String , n2: &'a String) -> &'a String{
// fn max_length_string<'a ,'b:'a >(n1: &'a String , n2: &'b String) -> &'a String{
    if n1.len() > n2.len(){
        return n1;
    }
    else{
        return n2;
    }
}

//here 'a:'b means lifetime 'a must live at least as long as lifetime 'b
fn get_length<'a:'b,'b>(n1:&'a String, n2:&'b String) -> &'b String{
    if n1.len() > n2.len(){
        return n1;
    }
    return n2;   
}

fn get_length2<'a,'b:'a>(n1:&'a String, n2:&'a String, n3:&'b String)-> &'a String{

    if n1.len() > n2.len() && n1.len() > n3.len(){
        return n1;
    }
    else if n2.len()> n3.len(){
        return n2;
    }
    return n3;
}


