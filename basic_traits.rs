// Basic of Traits With Rust programing Language
trait Shape {
    fn area(&self) -> f32;
    fn print_area(&self);
}
struct Rectangle{
    width:f32,
    height:f32,
}
struct Triangle {
    base:f32,
    height:f32,
}
impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.height * self.width
    }
    fn print_area(&self) {
        println!("Area Of Rectangle : {}",self.area());
    }
}
impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.base * self.height * 0.5
    }
    fn print_area(&self) {
        println!("Area Of Trangle : {}",self.area());
    }
}
fn main() {
    let rec = Rectangle{
        width : 3.2,
        height : 5.4,
    };
    rec.area();
    rec.print_area();

    let triangle = Triangle{
        base : 3.2,
        height : 5.4,
    };
    triangle.area();
    triangle.print_area();
}
