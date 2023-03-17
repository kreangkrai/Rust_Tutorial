//Find Most Common Word is not Banned.
use std::collections::HashMap;
struct Solution{}
#[allow(dead_code)]
impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let para = paragraph.replace(|c: char| !c.is_ascii() | !c.is_alphabetic(), " ").to_lowercase();
        let mut hm:HashMap<&str, i32> = HashMap::new();
        let temp: Vec<&str> = para.split_whitespace().collect();
        for p in temp.into_iter() {
            let check = banned.iter().find(|&x| x == p);
            match check {
                None => {
                    *hm.entry(p).or_insert(0) += 1;
                }
                Some(_) => {}
            }
        }
        hm.into_iter()
        .max_by(|a,b| a.1.cmp(&b.1))
        .map(|(key,_)| key).unwrap().to_owned()
    }
}
fn main() {
    
}
#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test_most_common_word(){
        assert_eq!(Solution::most_common_word(String::from("Bob hit a ball, the hit BALL flew far after it was hit."), vec![String::from("hit")]),"ball");
        assert_eq!(Solution::most_common_word(String::from("a."), vec![]),"a");
    }
}
