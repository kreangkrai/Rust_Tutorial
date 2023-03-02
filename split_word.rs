use regex::Regex;
fn main() -> Result<(),regex::Error>{
     let text = "Crypto.BTC/USD,Crypto.ETH/USD,Crypto.SOL/USD";
     let re = Regex::new(r"\.(?P<name>\w+)/")?;
     let res = re.captures_iter(text);
     for mat in res{
          println!("Name : {:?}",&mat["name"]);        
     }
     Ok(())
}
============================================================
 fn main() {
    let text = "Crypto.BTC/USD,Crypto.ETH/USD,Crypto.SOL/USD";
    let word:Vec<&str>  = text
                         .chars()
                         .enumerate()
                         .filter(|(_,c)| *c == '.')
                         .map(|(i,_)|{
              let slash = text[(i+1)..].find('/').unwrap();                                                                         &text[(i+1)..(i + slash +1)] as &str
    }).collect();
    println!("{:?}",word);
}
