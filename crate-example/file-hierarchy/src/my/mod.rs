mod inaccessible;

pub mod nested;

pub fn function() {
    println!("called `my::fucntion()`");
}

fn private_funcion() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    println!("called `my::indirect_access()`, that \n> ");
    private_funcion();
}