// use std::ops::Mul;
struct Rect<T>{
    height:T,
    width:T,
}

// impl<T: Mul<Output = T> + Copy> Rect<T>{
//implementing Ret for generic
//trait bound with operation(ops)
impl<T: std::ops::Mul<Output = T> + Copy> Rect<T>{

    fn area(&self)->T{

        self.height * self.width
    }
}

fn main(){
    let r1 = Rect{
        height:10,
        width:5
    };
    println!("{}",r1.area());

    let r2 = Rect{
        height:10.9,
        width:5.4
    };
    println!("{}",r2.area());

}