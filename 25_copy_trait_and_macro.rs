//Copy macro trait example
//Types that implement Copy trait do not move ownership, instead they get copied
//Types that implement Copy trait are usually primitive types like integers, floats, booleans,
//characters, and tuples of Copy types
//If a struct only contains fields that are Copy types, then the struct automatically implements
//the Copy trait
#[derive(Debug,Copy,Clone)]
struct User{
    is_male:bool,
    age:u8,
}

fn main(){
    let u1 = User{
        is_male:true,
        age:26,
    };
    let u2 = u1; // because User struct only contains Copy types(primitive types), so it implements Copy trait by default
    println!("{:?},{:?}",u1,u2);
}