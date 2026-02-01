#[derive(Debug)]
struct User<'a> {
    name: &'a str
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name }
    }
}
fn main() {
   let name = String::from("John");
   let u1 = User{
    name: &name,
   };
   let user = User::new(&name);
   println!("{:?}", u1);
   println!("{}", user.name);
}
