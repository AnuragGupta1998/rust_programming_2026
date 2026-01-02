fn main(){
    println!("Hello, world! main function executed.");

    //ownership example with primitive types it values are stored into stack memory
    let a:u8 = 10;
    let b:u8 =a;

    println!("Value of a: {}, Value of b: {}", a, b); //a=10,b=10

    //ownership example with non-primitive types like String values are stored into heap memory
    let str1 :String = String::from("Hello");

    let str2 = str1; //ownership of str1 is moved to str2
    println!("Value of str2: {}", str2); //str2=Hello
    //println!("Value of str1: {}", str1); //This line would cause a compile-time error because str1 has been moved to str2

    
    //transferring ownership back to the caller using functions
    let str3 :String = String::from("World Anurag");   
    let (str3,l) = transfer_ownership(str3);   //here ownership of str3 is moved to the function and then returned back
    println!("Value of str3 after transferring ownership: {},{}", str3,l); //str3=World

    let x:u8 =20;
    process_value(x); //x is copied to the function as u8 is a primitive type
    println!("Value of x after process_value function call: {}", x); //x=20
}

fn transfer_ownership(s:String) -> (String,usize) { //s comes into scope and now ownwership of s is with this function
    println!("Inside transfer_ownership function: {}", s);
    let length = s.len();
    (s,length) //it returns a tuple containing the String and its length
    // return (s,length); //returning ownership back to the caller
}

fn process_value(num:u8) { //num comes into scope and now ownwership of num is with this function
    println!("Inside process_value function: {}", num); //num =20
    //num goes out of scope here, but since u8 is a primitive type, nothing special happens
}