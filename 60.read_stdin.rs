//Basic Read Data from stdin

use std::io::{stdin, Result};
use std::ops::Add;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x: x, y: y }
    }
}
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl<T: Default> Default for Point<T> {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
fn main() -> Result<()> {
    let mut point = Point::default();
    let mut n = String::new();
    println!("Input Loop for Push Data : ");
    stdin().read_line(&mut n)?;
    let k: i32 = n.trim().parse().expect("Input not an integer");
    let mut m = 0;
    loop {
        if m == k {
            break;
        }
        println!("Enter Data Point {} format [x:?,y:?]", m + 1);
        let mut input_point = String::new();
        stdin().read_line(&mut input_point)?;
        let data = input_point
            .trim()
            .split(",")
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        let mut x = 0;
        let mut y = 0;
        if let Ok(n) = data[0]
            .trim()
            .split(":")
            .map(|f| f.parse::<i32>())
            .collect::<Vec<_>>()[1]
        {
            x = n;
        }
        if let Ok(n) = data[1]
            .trim()
            .split(":")
            .map(|f| f.parse::<i32>())
            .collect::<Vec<_>>()[1]
        {
            y = n;
        }
        point = point + Point::new(x, y);
        m += 1;
    }
    println!("Sum : {:?}", point);

    Ok(())
}
