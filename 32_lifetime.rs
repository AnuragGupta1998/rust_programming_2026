fn main() {
    let str1 = String::from("oneiiii999999");
    let str2 = String::from("two");
    let result;

    {
        let str3 = String::from("three");
        result = longest_str(&str1, &str2, &str3);
    }
    println!("{}", result);
}

fn longest_str<'a, 'b>(s1: &'a String, s2: &'a String, s3: &'b String) -> &'a String {
    if s1.len() > s2.len() && s1.len() > s3.len() {
        return s1;
    }
    return s2;
}