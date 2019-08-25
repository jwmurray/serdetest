use serde::*;
use serde_derive::*;
use serde_json::*;

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    status: String,
    content: String
}

fn main() {

    let greeting = Greeting { status: String::from("success"), content: String::from("Hello World") };
    let serialized = serde_json::to_string(&greeting).unwrap();
    println!("Serialized: {}", serialized);


    let deserialized: Greeting = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}
