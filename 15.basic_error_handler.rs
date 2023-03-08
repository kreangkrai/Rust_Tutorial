use std::{fmt::Display, error::Error};
#[derive(Debug)]
enum MyError{
    Error(&'static str),
}
#[derive(Debug)]
enum Result<T,E>{
    Err(E),
    Ok(T),
}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Error(e) => write!(f,"{}",e)
        }
    }
}
impl Error for MyError {}
fn divide(dividend: f32, divisor: f32)-> Result<f32,MyError>{
    if divisor == 0f32{
        Result::Err(MyError::Error("Divide By Zero"))
    }else{
        Result::Ok(dividend / divisor)
    }
}
fn main(){
    let dividend = 5.0;
    let divisor = 0.0;

    let answer = divide(dividend, divisor);
    println!("{:?}",answer);
    
}
