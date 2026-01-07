//struct definition
struct Rectangle {
    width:u32,
    height:u32,
}

fn main(){
    //creating immutable instance of struct
    let r1 = Rectangle {
        width:10,
        height:50,
    };
    println!("Width: {}, Height: {}", r1.width, r1.height);
    //r1.height=10; //this will give error as r1 is immutable by default in rust    

    let area_of_r1:u32=r1.height*r1.width;
    println!("Area of r1: {}", area_of_r1);

    //mutable instance of struct that can chnage struct fields
    let mut r2 = Rectangle {
        width:10,
        height:40,
    };

    println!("Width: {}, Height: {}", r2.width, r2.height);

    r2.width = 25;
    r2.height = 60;
    println!("Updated Width: {}, Height: {}", r2.width, r2.height);

    let area_of_r2:u32=r2.height*r2.width;
    println!("Area of r2: {}", area_of_r2);
}