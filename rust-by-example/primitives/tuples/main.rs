use std::fmt::{Display, Formatter, Result};

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (interger, boolean) = pair;
    (boolean, interger)
}

fn transpose(matrix: Matrix) -> Matrix {
    let x1 = matrix.0;
    let x2 = matrix.1;
    let y1 = matrix.2;
    let y2 = matrix.3;

    Matrix(x1, y1, x2, y2)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        write!(f, "( {} {} )", self.2, self.3) 
    }
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    let tuple_of_tuple = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuple: {:?}", tuple_of_tuple);
    
    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reverse pair is {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("Transpose:\n{}", transpose(matrix));
}
