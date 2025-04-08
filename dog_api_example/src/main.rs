fn serde_serialize() {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize)]
    struct Person {
        name: String,
        age: u8,
    }
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
    };
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized Person = {}", serialized);
    println!("{:?}",person);
}

fn serde_deserialize(){
    use serde::{Serialize, Deserialize};
    #[derive(Deserialize)]
    struct Person {
        name: String,
        age: u8,
    }
    let data = r#"{"name": "John Doe", "age": 30}"#;
    println!("{}", data);

    let person: Person = serde_json::from_str::<Person>(data).unwrap();
    println!("Deserialized Person = {}, {}", person.name, person.age);
}

fn main() {
    serde_serialize();
    serde_deserialize();
}
