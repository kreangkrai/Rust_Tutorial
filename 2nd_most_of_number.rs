// Find 2nd most of number and tell quantity in multi vector With Rust Programing Language
use std::collections::HashMap;
fn main() {
    let v1 = vec![1,2,3,4,5];
    let v2 = vec![9,-1,3,-8,11];
    let v3 = vec![11,-2,0,1,9];
    let mut hm:HashMap<i32,i32> = HashMap::new();
    let v = v1.into_iter()
                                       .chain(v2.into_iter())
                                       .chain(v3.into_iter());
    for n in v.into_iter(){
        *hm.entry(n).or_insert(0) +=1;
    }

    let mut hm_vec:Vec<(i32,i32)> = hm.into_iter().collect();
    hm_vec.sort_by(|a,b| b.0.cmp(&a.0));

    println!("Find 2nd most of number");
    println!("Number : {} , Quantity : {}",hm_vec[1].0 , hm_vec[1].1);
}
