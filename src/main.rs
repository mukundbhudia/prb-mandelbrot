extern crate num;
use num::Complex;

fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
    let mut z = Complex { re: 0.0, im: 0.0 };   // We define a struct (structure type) for a complex number
    loop {
        z = z * z + c;
    }
}


/// Try to find out if 'c' is in the Mandelbrot set, using a max of 'limit' iterations to decide
/// 
/// If 'c' is not a member, return 'Some(i)' where 'i' is the number of iterations taken for 'c' to leave
/// the circle of radius 2 at the origin. If 'c' seems to be a member (or more precisley, if we reached the 
/// iteration limit without being able to prove 'c' is not a member), return 'None'.
#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {        // Option is an enum, and enumerated type
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}
