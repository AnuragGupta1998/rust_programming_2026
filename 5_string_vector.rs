fn main(){
    println!("this is a string and vector example in rust");

    // Example of String and Vector declaration and usage
    let name:String = String::from("Anurag");

    println!("String value: {}", name);
    
    name.push_str("Gupta");
    println!("Updated String value: {}", name);


    let v :Vec<i32> = vec![10,11,12,13,14,15];
    println!("Vector values: {:?}", v);
    println!("Vector length: {}", v.len());
    println!("First element in Vector: {}", v[0]);
    println!("last element in Vector: {}", v[v.len()-1]);

    let arr:[i32;5] =[1,2,3,10,20];
    println!("Array values: {}", arr.len());
    println!("Array values: {:?}", arr.len());
    println!("Array values: {:?}", arr);
    println!("First element in Array: {}", arr[0]);
    println!("Last element in Array: {}", arr[arr.len()-1]);
}
