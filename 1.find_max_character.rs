// Find max character in string With Rust Programing Language

use std::collections::HashMap;
#[derive(Debug)]
#[allow(dead_code)]
struct Data {
    char: char,
    value: i32,
}
fn main() {
    let str = String::from("abcbsbdbaaabsbsbabaaasa");
    let mut m: HashMap<char, i32> = HashMap::new();
    for s in str.chars().into_iter() {
        *m.entry(s).or_insert(0) += 1;
    }

    let max_char = m
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(key, val)| Data {
            char: key,
            value: val,
        }).unwrap();
    println!("{:#?}", max_char);
}
