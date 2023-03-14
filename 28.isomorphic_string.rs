//Check Isomorphic String With Rust Programing Language

use std::collections::{HashMap,HashSet};
pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len(){
        return false;
    }
    let mut hm:HashMap<u8,u8> = HashMap::new();
    for n in 0..s.as_bytes().len(){
        hm.entry(s.clone().into_bytes()[n]).or_insert(t.clone().into_bytes()[n]);
        let val = hm.get(&s.clone().into_bytes()[n]);
        match val {
            Some(v) =>{
                if *v != t.clone().into_bytes()[n] {
                    return false;
                }
            },
            None =>{}
        }        
    }
    let len_key = hm.keys().collect::<HashSet<&u8>>().len();
    let len_val = hm.values().collect::<HashSet<&u8>>().len();
    len_key == len_val
}
fn main() {
    let s = String::from("badc");
    let t = String::from("baba");
    let b = is_isomorphic(s, t);
    println!("{:?}",b);
}
