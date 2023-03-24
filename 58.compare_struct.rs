//Compare Two Struct
#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T,
}
impl <T> Point<T> {
    fn new(x:T,y:T)->Self{
        Point { x: x, y: y }
    }
}
impl <T:PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
fn main() {
    let p1 = Point::new(1,2);
    let p2 = Point::new(1,2);

    let p = p1 == p2;
    println!("{}",p);
}
