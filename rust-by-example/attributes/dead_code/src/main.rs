
fn used_function() {}

#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}


#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("you are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("you are not running linux");
}

#[cfg(some_condition)]
fn conditional_funcion() {
    println!("condition met!");
}

fn main() {
    println!("Hello, world!");
    used_function();
    are_you_on_linux();
    conditional_funcion();
}
