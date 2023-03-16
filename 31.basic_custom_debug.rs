//Basic Custom Debug for struct
use std::fmt::Debug;
use std::ops::Add;
#[derive(Clone)]
struct Point{
    x:i32,
    y:i32,
}
impl Point {
    fn new(x:i32,y:i32)->Self{
        Point { x: x, y: y }
    }
}
impl Add for Point{
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point{
            x: self.x + other.x,
            y:self.y + other.y,
        }
    }
}
impl Debug for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point").field("X", &self.x).field("Y", &self.y).finish()
    }
}
#[derive(Clone)]
struct Points{
    points:Vec<Point>,
}
impl Points {
    fn new()->Self{
        Points { points: Vec::new() }
    }
    fn insert(&mut self,point:Point){
        self.points.push(point);
    }
    fn sum(&self)->Point{
        self.points.clone().into_iter().fold(Point::new(0,0), |acc,p| acc+p)
    }
}
impl Debug for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Points").field("Data", &self.points).finish()
    }
}
fn main() {
   let mut points:Points = Points::new();
   points.insert(Point::new(3, 4));
   points.insert(Point::new(5, 6));
   points.insert(Point::new(1, 3));
   println!("Data {:#?}",points);
   println!("Sum : {:?}",points.sum());
}
