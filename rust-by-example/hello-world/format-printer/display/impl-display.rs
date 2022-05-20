use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}


fn main() {
    println!("{}", Structure(3));
    
    let minmax = MinMax(-100, 200);

    println!("Compare structure");
    println!("Display minmax: {}", minmax);
    println!("Debug minmax: {:?}", minmax);
    

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("the big range is {big} and small range is {small}",
        small = small_range, 
        big = big_range);


    let point = Point2D {x: 3.3, y: 7.2};


    println!("Compare Point2D");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);


    let complex = Complex { real: 3.3, imag: 7.2 };
    println!("Activity");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);


}
