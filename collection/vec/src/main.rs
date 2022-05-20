
enum SpreadsheeetCell {
    Int(i32),
    Float(f64),
    Text(String),
}




fn main() {
    let mut v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    v.push(10);


    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element."),
    }

    let dose_not_exist = v.get(100);

    let first = &v[0];
    println!("the first element is: {}", first);

    for i in &mut v {
        *i += 50;
    }

    for i in v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheeetCell::Int(3),
        SpreadsheeetCell::Text(String::from("blue")),
        SpreadsheeetCell::Float(10.121);
    ]
}
