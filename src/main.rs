extern crate num;
use num::Complex;
use std::str::FromStr;

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

/// Try to find out if `c` is in the Mandelbrot set, using a max of `limit` iterations to decide
/// 
/// If `c` is not a member, return `Some(i)` where `i` is the number of iterations taken for `c` to leave
/// the circle of radius 2 at the origin. If `c` seems to be a member (or more precisley, if we reached the 
/// iteration limit without being able to prove `c` is not a member), return `None`.
#[allow(dead_code)]
fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {        // Option is an enum, and enumerated type
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {     // Computing the square of z is faster than computing the square root
            return Some(i);
        }
    }

    None
}

/// Parse the string `s` as a coordinate pair, like "400x600" or "1.0,0.5".
///
/// Specifically `s` should have the form <left><sep><right> where <sep> is the character given by the 
/// `seperator` argument, and <left> and <right> are both strings that can be parsed by `T::from_str`.
/// 
/// If `s` has the proper form, return `Some <(x,y)>` otherwise if it does not parse, return `None`.
#[allow(dead_code)]
fn parse_pair<T: FromStr>(s: &str, sperarator: char) -> Option<(T, T)> {
    match s.find(sperarator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}
