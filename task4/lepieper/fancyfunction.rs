fn main() {
    println!("{:?}", sumandproduct(4, 6))
}

use std::ops::{Add, Mul};
fn sumandproduct<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    (a + b, a * b)
}
