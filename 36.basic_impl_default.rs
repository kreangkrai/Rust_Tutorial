//Basic Implement Default for Struct
use std::ops::Add;
#[derive(Debug)]
struct Point {
    x:i32,
    y:i32,
}
impl Point {
    fn new(x:i32,y:i32)->Self{
        Point { x: x, y: y }
    }
}
impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}
impl Default for Point {
    fn default() -> Self {
        Point { x: 1, y: 1 }
    }
}
fn main() {
    let p1 = Point::new(3, 4);
    let p2 = Point::new(5,6);
    let p = p1 + p2 + Default::default();
    println!("Sum : {:?}",p);
}
