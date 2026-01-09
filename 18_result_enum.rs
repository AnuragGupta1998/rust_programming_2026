use std::fs;

fn main(){

    //a.txt should present at root directory,outside src folder
    let content = fs::read_to_string("a.txt");

    match content {

        //if content is Ok then print the actual content present in a.txt file
        Ok(data) =>println!("the data present in a.txt= {}", data),

        // if content is Err then print the error message
        Err(e) => println!("error while reading from file {}",e)
    }
}