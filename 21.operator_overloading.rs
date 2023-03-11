//Basic Operator Overloading With Rust Programing Language
use std::ops::{Add, Div, Mul, Sub};
#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl<T: Mul<Output = T>> Mul for Point<T> {
    type Output = Point<T>;

    fn mul(self, other: Self) -> Self::Output {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
impl<T: Div<Output = T>> Div for Point<T> {
    type Output = Point<T>;

    fn div(self, other: Self) -> Self::Output {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}
fn main() {
    let p1 = Point::new(3.22, 5.42);
    let p2 = Point::new(1.61, 2.71);

    let add_p = p1.clone() + p2.clone();
    let sub_p = p1.clone() - p2.clone();
    let mul_p = p1.clone() * p2.clone();
    let div_p = p1.clone() / p2.clone();
    println!("Add {:?}", add_p);
    println!("Sub {:?}", sub_p);
    println!("Mul {:?}", mul_p);
    println!("Div {:?}", div_p);
}
