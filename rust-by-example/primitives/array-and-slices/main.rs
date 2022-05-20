use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 1000] = [0; 1000];

    println!("first element of array: {}", xs[0]);
    println!("second element of array: {}", xs[1]);
    println!("number element of array: {}", xs.len());

    println!("array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&ys[1..10]);

    println!("{}", xs[4]);
    println!("{}", ys[1]);
}
