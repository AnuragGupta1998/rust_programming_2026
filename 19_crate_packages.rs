use chrono::prelude::*;
use chrono::{Utc,Local};
use dotenv::dotenv;

fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let var = env::var("REDIS_ADDRESS").unwrap();
    println!("{}", var);
    
    let var = env::var("REDIS_ADDRESS");

    match var{
        Ok(p)=>println!("return from env file {}",p),
        Err(e) => println!("the error is {}",e),
    }

    // let utc_time = Utc::now().date_naive();
    let utc_time = Utc::now();
    println!("the utc time is {} ",utc_time);

    //the own local time
    let local_time = Local::now();
    println!("the local time is {}",local_time);
}
