

fn main() {
    let x: &String;
    
    {
        let y = String::from("heeh");
        x = &y;
        println!("{}, {}", y, x);
    }

}
