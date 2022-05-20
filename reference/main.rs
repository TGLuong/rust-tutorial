
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    println!("{:?}", bytes);
    println!("{:?}", bytes.iter().enumerate());
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("gia luong");
    let world = first_word(&s);

    s.clear();

    println!("{}", world);
}