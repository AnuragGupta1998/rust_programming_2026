fn main(){
//     let outer;
//  {
//     let inner = 5;
//     outer = &inner; // outer is a reference to inner and inner does not live long enough which cauch dangling reference error
//  }
//     println!("The value of outer is: {}", outer);



 let x = 50;
 let y = 10;

 let result  = larger_value(&x,&y);
 println!("The larger value is: {}", result);
} 

fn larger_value<'a> (a: &'a i32 ,b: &'a i32) -> &'a i32{
    // if a<b {b} else{a}

    //OR
    if a < b {
       return b;
    }
    return a;
}