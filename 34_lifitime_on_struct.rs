#[derive(Debug)]
struct User<'a>{
    name:&'a String,
    age:u8
}

fn main(){
    let n = String::from("Anurag");
    let u1= User{
        name:&n,
        age:26
    };
    println!("Name: {}, Age: {}", u1.name, u1.age);
    println!("{:?}",u1);
    println!("User details: {}{}", u1.name,u1.age);

}