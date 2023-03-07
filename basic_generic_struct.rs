//struct implementing the Add trait using generics
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
impl <T:Add<Output=T>> Add<Point<T>> for Point<T>{
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Self::Output {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}
fn main() {
   let p1 :Point<f32> = Point::new(1.1, 2.5);
   let p2 :Point<f32> = Point::new(1.5, 2.8);

   let p = p1 + p2;
   println!("{:?}",p);
}
