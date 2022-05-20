fn main() {
    let number = 5;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition: bool = true;

    let num = if condition { "five" } else { "six" };
    println!("The value of num is {}", num);

    let mut count = 0;
    let mut result = 0;
    result = 'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up 10 * 2;
            }
            remaining -= 1;
        }

        count += 1;
    };

    println!("End count {}", count);
    println!("Result {}", result);
}
