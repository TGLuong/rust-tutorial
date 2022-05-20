
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run (&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let x = Operations::Add;
    let y = Operations::Subtract;

    println!("{}", x.run(10, 20));
    println!("{}", y.run(10, 20));
}

