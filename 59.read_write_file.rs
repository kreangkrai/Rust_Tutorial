//Basic Read Write File

use std::{fs::File,io::{self,BufRead,Result,Write}};
use std::ops::Add;

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
impl <T:Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}
impl <T:Default> Default for Point<T> {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}
fn main() -> Result<()> {
    let mut point = Point::default();

    //read file
    let file = File::open("data.txt")?;
    let datas = io::BufReader::new(file).lines();

    for line in datas{
        match line {
            Ok(l) => {
                let vec = l.split(",").map(|f| 
                    f.to_string()).collect::<Vec<String>>();
                let mut x = 0;
                let mut y = 0;
                if let Ok(n) = vec[1].split(":").map(|f| 
                    f.parse::<u8>()).collect::<Vec<_>>()[1]{
                        x = n;
                }
                if let Ok(n) = vec[2].split(":").map(|f|
                     f.parse::<u8>()).collect::<Vec<_>>()[1]{
                        y = n;
                }
                point = point + Point::new(x,y);              
            },
            Err(_) =>{}
        }
    }
    //write file
    let mut file = File::create("output.txt")?;
    file.write_all(format!("x:{},y:{}",point.x,point.y).as_bytes())?;
    Ok(())
}
