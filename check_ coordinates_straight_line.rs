//Check Coordinates is Straight Line With Rust Programing Language

/*
   [[1,2],[2,3],[3,4],[4,5],[5,6],[6,7]]  = True
Y
|
|                 o (6,7)
|              o (5,6)
|           o (4,5)
|        o (3,4)
|     o (2,3)
|  o (1,2)
|
|_________________________ X

   [[1,1],[2,2],[3,4],[4,5],[5,6],[7,7]] = False
Y
|                   o (7,7)
|              o (5,6)   
|           o (4,5)  
|        o (3,4)     
|        
|     o (2,2)
|  o (1,1)
|_________________________ X
 */
#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
#[allow(dead_code)]
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}
#[derive(Debug, Clone)]
struct Coordinates {
    points: Vec<Point>,
}
#[allow(dead_code)]
impl Coordinates {
    fn new() -> Self {
        Coordinates { points: Vec::new() }
    }
    fn push(&mut self, point: Point) {
        self.points.push(point);
    }
    fn convert(&self) -> Vec<Vec<i32>> {
        let mut coordinates: Vec<Vec<i32>> = Vec::new();
        for p in self.points.clone().into_iter() {
            coordinates.push(vec![p.x, p.y]);
        }
        return coordinates;
    }
    fn check_straight_line(&self) -> bool {
        let coordinates = self.convert();
        let i = coordinates[0][0];
        let j = coordinates[0][1];
        if let true = coordinates.iter().all(|x| x[0] == i) {
            return true;
        }
        if let true = coordinates.iter().all(|x| x[1] == j) {
            return true;
        }
        let mut y = coordinates[1][1] - coordinates[0][1];
        let mut x = coordinates[1][0] - coordinates[0][0];
        if x == 0 {
            return false;
        }
        let m: f32 = y as f32 / x as f32;

        for i in 1..coordinates.len() - 1 {
            y = coordinates[i + 1][1] - coordinates[i][1];
            x = coordinates[i + 1][0] - coordinates[i][0];
            if x == 0 {
                return false;
            }
            if m != y as f32 / x as f32 {
                return false;
            }
        }
        true
    }
}
fn main() {
}

#[cfg(test)]
mod tests {
    use crate::Coordinates;
    use crate::Point;
    #[test]
    fn test_straight_line1(){
        let mut coordinates = Coordinates::new();
        coordinates.push(Point::new(1, 2));
        coordinates.push(Point::new(2, 3));
        coordinates.push(Point::new(3, 4));
        coordinates.push(Point::new(4, 5));
        coordinates.push(Point::new(5, 6));
        coordinates.push(Point::new(6, 7));
        assert_eq!(coordinates.check_straight_line(),true);
    }
    #[test]
    fn test_straight_line2(){
        let mut coordinates = Coordinates::new();
        coordinates.push(Point::new(1, 1));
        coordinates.push(Point::new(2, 2));
        coordinates.push(Point::new(3, 4));
        coordinates.push(Point::new(4, 5));
        coordinates.push(Point::new(5, 6));
        coordinates.push(Point::new(7, 7));
        assert_eq!(coordinates.check_straight_line(),false);
    }
}
