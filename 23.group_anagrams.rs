//Group Anagrams in Vector String With Rust Programing Language
use std::collections::HashMap;
fn group_anagrams(strs:Vec<&str>)->Vec<Vec<&str>>{
    let mut hm:HashMap<[u32;26],Vec<&str>> = HashMap::new();
    for s in strs.into_iter(){
        let mut key:[u32;26] = [0;26];
        for ch in s.chars().into_iter(){
            key[(ch as u32 - 'a' as u32) as usize] +=1;
        }
        hm.entry(key).or_insert(Vec::new()).push(s);
    }
    hm.into_iter().map(|(_,val)| val).collect()
}
fn main(){
   let strs = vec!["eat","tea","tan","ate","nat","bat"];
   let answer = group_anagrams(strs);
   println!("{:?}",answer);

   let strs = vec![""];
   let answer = group_anagrams(strs);
   println!("{:?}",answer);

   let strs = vec!["a"];
   let answer = group_anagrams(strs);
   println!("{:?}",answer);
}
