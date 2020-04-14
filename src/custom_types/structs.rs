#[derive(Debug)]
struct Person {
    id: i64,
    name: String,
    age: i16,
    address: String,
}

fn build_person() -> Person {
    Person {
        id: 10,
        name: String::from("xyz_name"),
        address: String::from("xyz address"),
        age: 12,
    }
}
pub fn access_structs() {
    let x: Person = build_person();
    println!("{}, {}, {}, {}", x.id, x.address, x.age, x.name);
    println!("Only allowed if decorated with Debug derive {:?}", x);
}
