fn main() {
    println!("{:?}", sumandproduct(4, 6))
}

use std::ops::Add;
use std::ops::Mul;
use std::marker::Copy;
fn sumandproduct<T: Add + Mul + Copy>(a: T, b: T) -> (<T as Add>::Output, <T as Mul>::Output) {
    let v: (<T as Add>::Output, <T as Mul>::Output) = (a + b, a * b);
    v

}
