//Basic Deref,DerefMut
use std::ops::{Deref,DerefMut};
#[derive(Debug)]
struct Point<T>{
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)->Self{
        Point { x: x, y: y }
    }
}
#[derive(Debug)]
struct Points<T>{
    points:Vec<T>,
}
impl <T> Points<T> {
    fn new()->Self{
        Points { points: Vec::new() }
    }
}
impl <T> Deref for Points<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.points
    }
}
impl <T> DerefMut for Points<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points
    }
}
fn main() {
    let mut points:Points<Point<i32>> = Points::new();
    points.push(Point::new(1, 2));
    println!("{:?}",*points);

    points[0] = Point::new(9, 8);
    println!("{:?}",*points);
    points[0].x = 88;
    points[0].y = 99;
    println!("{:?}",*points);
}
