use std::ops::{Add, Mul};

fn main() {
    println!("{:?}", sumandproduct(4, 6))
}

fn sumandproduct<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (a + b, a * b)
}
