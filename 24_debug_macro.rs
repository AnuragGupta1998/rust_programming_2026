#[derive(Debug)]

struct User{
    name:String,
    password:String,
    age:u32
}

fn main(){
    let u1 = User{
        name:String::from("Anurag"),
        password:String::from("Anurag123"),
        age:26,
    };

    println!("user details: {:?}",u1);

    let v = vec![10,20,30,40,50];
    println!("vector details: {:?}",v); //[10, 20, 30, 40, 50]
}