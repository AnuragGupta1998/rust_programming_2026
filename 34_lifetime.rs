fn main(){
    let str1=String::from("hello");
    let ans;

    {
        let str2=String::from("world");
        ans=larget_str(&str1,&str2);
        println!("{}",ans);
    }
    // println!("{}",ans);
}
fn larget_str<'a,'b:'a>(s1: &'a String , s2: &'b String) -> &'a String {
    // if s1.len() > s2.len(){s1}else{s2}
    if s1.len() >=  s2.len(){
       return s1;
    }
    return s2;
}