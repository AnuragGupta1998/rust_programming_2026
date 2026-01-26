use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug,PartialEq)]
struct User{
    name:String,
    amount:u32,
    // data:Vec<u8>,
}

fn main() {
    println!("Borsh Library for rust and solana development at the end!");

    let original_user = User{
        name:String::from("AnuraG"),
        amount :500,
        // data:vec![1,2,3,4,5],
    };

    // store_bytes the serialized data byte array 
    let mut store_bytes = Vec::new(); 

    // Serialize the user struct to a byte array
    //stored bytes into store_bytes vector variable
    original_user.serialize(&mut store_bytes).unwrap();
    println!("Serialized Data: {:?}", store_bytes);

    //deserialize the byte array back to a user struct
    //try_from_slice convert q byte array to User struct
    let deserialize_data = User::try_from_slice(&store_bytes).unwrap();
    println!("Deserialized Data: {:?}", deserialize_data);

    assert_eq!(original_user, deserialize_data, "Original and Deserialized data do not match!");
}
