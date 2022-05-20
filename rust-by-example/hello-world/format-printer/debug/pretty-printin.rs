#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    let name = "Gia Luong";
    let age = 22;
    let luong = Person{ name, age };
    println!("{:#?}", luong);
}
