//Basic Drop
#[derive(Debug,Clone)]
#[allow(dead_code)]
struct Point {
    x:i32,
    y:i32,
}
impl Point {
    fn new(x:i32,y:i32)->Self{
        Point { x: x, y: y }
    }
}
impl Drop for Point {
    fn drop(&mut self) {
        println!("dropped => {:?}",self);
    }
}
#[derive(Debug,Clone)]
struct Points{
    points:Vec<Point>,
}
impl Points {
    fn new() ->Self{
        Points { points: Vec::new() }
    }
    fn push(&mut self,value:Point){
        self.points.push(value);
    }
}
fn main() {
    let mut p = Points::new();
    p.push(Point::new(1,2));
    p.push(Point::new(3,4));
    println!("{:?}",p);
    println!("============ Drop ============");
}
