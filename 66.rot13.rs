//ROT13
#[derive(Debug)]
struct ROT13 {
    str:String,
}
impl ROT13 {
    fn new(str:String)->Self{
        ROT13 { str: str }
    }
    fn is_chars(&self)->bool{
        let mut b = false;
        for s in self.str.clone().chars(){
            if (s as u8 >= 65 && s as u8 <= 90) || (s as u8 >= 97 && s as u8 <= 122)  {
                b = true;
            }else{
                return false;
            }
        }
        return b;
    }
    fn encrypt(&self) ->Self{
        let str = self.str.clone();
        let mut res = String::new();
        if self.is_chars(){
            for s in str.chars(){
                if s.is_lowercase(){
                    let u_s = s as u8 + 13;
                    if u_s >= 97 && u_s <= 122 {
                        res.push_str(std::str::from_utf8(&[u_s]).unwrap());
                    }else{
                        res.push_str(std::str::from_utf8(&[u_s - 26]).unwrap());
                    }
                }else{
                    let u_s = s as u8 + 13;
                    if u_s >= 65 && u_s <= 90 {
                        res.push_str(std::str::from_utf8(&[u_s]).unwrap());
                    }else{
                        res.push_str(std::str::from_utf8(&[u_s - 26]).unwrap());
                    }
                }
            }
            ROT13 { str: res }
        }else {
            panic!("Error format");
        }
    }
    fn print_rot13(&self){
        println!("{:?}",self.str);
    }
}
fn main() {
    let rot13 = ROT13::new("HelLo".to_string());
    rot13.encrypt().print_rot13();
}
