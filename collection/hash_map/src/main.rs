use std::collections::HashMap;


fn main() {
    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10, 50];

    let mut socres: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", socres);



    let field_name = String::from("key");
    let field_value = String::from("value");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let team_name = String::from("key");
    println!("{:?}", map.get("key"));


    for (key, value) in &socres {
        println!("{}, {}", key, value);
    }

    println!("{:?}", socres);

    let mut override_map = HashMap::new();

    override_map.insert(String::from("blue"), 10);
    override_map.insert(String::from("blue"), 25);

    override_map.entry(String::from("blue")).or_insert(50);
    override_map.entry(String::from("yellow")).or_insert(30);

    println!("{:?}", override_map);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for world in text.split_whitespace() {
        let count = map.entry(world).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
