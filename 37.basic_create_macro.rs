//Basic Create Macro for Struct
use std::ops::Add;
#[derive(Debug,Clone)]
struct Point {
    x:i32,
    y:i32,
}
impl Point {
    fn new(x:i32,y:i32)->Self{
        Point{x:x,y:y}
    }
}
#[derive(Debug,Clone)]
struct Points{
    points:Vec<Point>,
}
impl Points {
    fn new()->Self{
        Points { points: Vec::new() }
    }
    fn push(&mut self,p:Point){
        self.points.push(p);
    }
    fn sum(&self)->Point{
        self.points.clone().into_iter().fold(Point { x: 0, y: 0 }, |acc,p| acc + p)
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

macro_rules! vec_point {
    ($($p:expr),+) => ({
        let mut v = Points::new();
        $( v.push($p); )+
        v
    });
}
fn main() {
    let v = vec_point![Point::new(3,4),Point::new(5,6)];
    println!("Data {:?}",v);
    println!("Sum {:?}",v.sum());
}
