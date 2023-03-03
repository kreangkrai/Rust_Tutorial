//Unit Test Function
//Find words that can be formed by characters With Rust Programing Language
use std::collections::HashMap;
pub struct Solution{}
impl Solution {
    pub fn count_chatacters(words:Vec<String>,chars:String) -> i32{
        let mut m:HashMap<char,i32> = HashMap::new();
        let mut count = 0;
        for i in chars.chars(){
            *m.entry(i).or_insert(0) +=1;
        }
        for i in 0..words.len(){
            let mut temp :HashMap<char,i32> = m.clone();
            for (j,val) in words[i].chars().enumerate(){
                if temp.contains_key(&val){
                    if temp.get(&val).unwrap() > &0{
                        *temp.entry(val).or_insert(0) -= 1;
                    }else{
                        break;
                    }
                    if j == words[i].len() -1{
                        count += words[i].len() as i32;
                    }
                }else{
                    break;
                }
            }
        }
        count
    }
}
fn main(){

}
#[cfg(test)]
mod tests{
    use crate::Solution;
    #[test]
    fn test_count_characters(){
        let words1 = vec![String::from("cat"),String::from("bt"),String::from("hat"),String::from("tree")];
        assert_eq!(Solution::count_chatacters(words1, String::from("atach")),6);

        let words2 = vec![String::from("hello"),String::from("world"),String::from("leetcode")];
        assert_eq!(Solution::count_chatacters(words2, String::from("welldonehoneyr")),10);
    }
}

// cargo test
