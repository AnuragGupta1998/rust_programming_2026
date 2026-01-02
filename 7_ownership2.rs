fn main(){
    let str1:String = get_str1();
    println!("the st1 from get_str1 is {}", str1); //str1 is owner of the string returned from get_str1 function  

    let str2:String = String::from("Anurag from main function"); //str2 owner of Anurag from main function
    let str3:String = send_get_string(str2); //it transfer ownership of str2 to the called function send_get_string
    println!("the str3 from get_str3_from_str2 is {}", str3);
}

fn send_get_string(name:String) -> String{
    name //it transfer ownership back to the caller function which is str3 in main function
}

fn get_str1() -> String {
    let s1 = String:: from ("Anurag from get_str1 function");
    s1 //it transfer ownership to the coller function which is str1 in main function
}