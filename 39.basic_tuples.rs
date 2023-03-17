//Basic Tuples
use  std::ops::Add;
#[derive(Debug,Clone)]
struct Point{
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
#[derive(Debug,Clone)]
struct Points {
    points:Vec<(Point,Point)>,
}
impl Points {
    fn new()->Self{
        Points { points: Vec::new() }
    }
    fn push(&mut self,p1:Point,p2:Point){
        self.points.push((p1,p2));
    }
    fn print(&self){
        for p in self.points.clone().into_iter(){
            println!("Tuple#1 {:?} , Tuple#2 {:?}",p.0,p.1);
        }   
    }
    fn sum(&self)-> (Point,Point){
        self.points.clone().into_iter()
            .fold((Point::new(0,0),Point::new(0,0)), |acc,p| (acc.0 + p.0 , acc.1 + p.1))
    }
}
fn main() {
    let mut p = Points::new();
    p.push(Point::new(3, 4), Point::new(5,6));
    p.push(Point::new(1, 3), Point::new(2,5));
    p.print();
    println!("Sum {:?}",p.sum());
}
