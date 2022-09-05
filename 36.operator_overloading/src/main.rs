#![allow(unused_mut)]

use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re: re, im: im }
    }
}

impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Complex<T>;
    // a + b
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// partial eq trait
// full eq: x = x (this cannot be satisified for general case because of the floating point numbers)
// NAN = not a number 0/0 inf/inf
// NAN == NAN -> false
impl<T> PartialEq for Complex<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.re == other.re && self.im == other.im
    }
}

// eq trait (no need for implementation since partial eq is implemented)
impl<T: Eq> Eq for Complex<T> {}

fn main() {
    let mut a = Complex::new(1., 2.);
    let mut b = Complex::new(3., 4.);

    // println!("{:?}", a + b);

    // a += b;
    // println!("{:?}", -a);
    println!("{}", a == b);
}

// PartialEq, PartialOrd ... can be derived directly next to Debug trait
