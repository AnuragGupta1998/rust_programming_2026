use serde::{Deserialize as DeserializeSerde, Serialize as SerializeSerde};

#[derive(SerializeSerde, DeserializeSerde, Debug, PartialEq)]
struct User {
    username: String,
    password: String
}

#[derive(Debug, PartialEq, SerializeSerde, DeserializeSerde)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>,
    user: User
}

fn main() {
    let s = MyStruct {
        id: 32,
        data: String::from("Harkirtat"),
        v: vec![1, 2, 3],
        user: User {
            username: String::from("Harkirtat"),
            password: String::from("Singh")
        }
    };
    let json_str = serde_yaml::to_string(&s).unwrap();
    print!("{}", json_str);
    let deserialized: MyStruct = serde_yaml::from_str(&json_str).unwrap();
    assert_eq!(deserialized, s);
    println!("done!")    
}
