//implementing Clone trait using derive macro to enable deep copy of struct instances

#[derive(Debug,Clone)]
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

    let u2 = u1.clone(); // using clone method to create a deep copy of u1
    let u2 = &u1; // using reference to borrow u1 without taking ownership

    println!("user1 details: {:?}",u1);
    println!("user2 details: {:?}",u2);
}