struct User{
    username:String,
}

fn main(){

    // println!("the sum is {}",sum(1,2));
    println!("the sum is {}",sum2(1,2));
    println!("the sum is {}",sum2(1.5,2.9));
    // println!("the sum is {}",sum2(true,false));
    // println!("the sum is {}",sum2(String::from("Anurag"),String::from("Kumar")));


    print_variables(1);//1
    print_variables(10.9);//10.9
    print_variables(true);//true
    print_variables(String::from("Anurag"));//Anurag
    let u1 = User{
        username:String::from("Ashutosh"),
    };
    print_variables(u1.username);


    let r1= biggest_element(1,9);
    println!("{}",r1);
}

//generic <T> it could be T,W,V
// fn sum<T>(a:T,b:T)->T{
//      return a+b;
// }

//adding trait  bound to vanished the error of T and T generic can not added
//trait bound
fn sum2<T: std::ops::Add< Output = T> > (a:T,b:T)->T{
    return a+b;
}

//Display crate bound
fn print_variables<T: std::fmt::Display> (a: T){
    println!("{}",a);
}

//Order crate bound only work for integer and u 
fn biggest_element<T: Ord>(a:T,b:T)->T{
    if a>b {
       return a;
    }
    b
}