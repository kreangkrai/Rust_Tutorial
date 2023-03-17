//Basic Closures
use std::ops::Add;
#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)-> Self{
        Point { x: x, y: y }
    }
}
impl <T:Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self::Output {
        Point{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}
fn main() {
    let p1 = |x:i32,y:i32| Point::new(x, y);
    let p2 = |x:i32,y:i32| Point::new(x, y);

    let p = p1(3,4) + p2(5,6);
    println!("Sum : {:?}",p);
}
