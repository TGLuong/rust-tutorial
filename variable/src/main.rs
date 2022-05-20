

const THREE_HOUT_IN_SECOND: u32 = 60 * 60 * 3; 

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    println!("const THREE_HOUT_IN_SECOND: {}", THREE_HOUT_IN_SECOND);

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x inner scope is : {}", x);
    }

    println!("The value of x is: {}", x);
}
