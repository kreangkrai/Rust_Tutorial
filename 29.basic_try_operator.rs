//Basic Try Operator With Rust Programing Language
#[derive(Debug)]
enum MyError{
    DivideByZero,
}
fn divide(dividend: f32, divisor: f32)-> Result<f32,MyError>{
    if divisor == 0f32{
        Err(MyError::DivideByZero)
    }else{
        Ok(dividend / divisor)
    }
}
fn main(){
    let dividend = 5.0;
    let divisor = 0.0;
    let answer = || -> Result<(),MyError>{
        let n = divide(dividend, divisor)?;
        println!("{:?}",n);
        Ok(())
    };
    if let Err(e) = answer(){
        println!("{:?}",e);
    }
}
