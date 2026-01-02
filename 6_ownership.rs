fn main(){
    println!("Hello, world! main function executed.");

    //ownership example with primitive types it values are stored into stack memory
    let a:u8 = 10;
    let b:u8 =a;

    println!("Value of a: {}, Value of b: {}", a, b); //a=10,b=10

    //ownership example with non-primitive types like String values are stored into heap memory
    let str1 :String = String::from("Hello");

    let str2 = str1;

    println!("Value of str2: {}", str2); //str2=Hello
    //println!("Value of str1: {}", str1); //This line would cause a compile-time error because str1 has been moved to str2

    
    //transferring ownership back to the caller using functions
    let str3 :String = String::from("World Anurag");   
    let (str3,l) = transfer_ownership(str3);   //here ownership of str3 is moved to the function and then returned back
    println!("Value of str3 after transferring ownership: {},{}", str3,l); //str3=World

}

fn transfer_ownership(s:String) -> (String,usize) { //s comes into scope and now ownwership of s is with this function
    println!("Inside transfer_ownership function: {}", s);
    let length = s.len();
    (s,length) //it returns a tuple containing the String and its length
    // return (s,length); //returning ownership back to the caller

}