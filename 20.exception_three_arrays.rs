//Exception of Three Arrays With Rust Programing Language

//[4, 9, 5, 8, 9, 4]
//[9, 4, 9, 8, 3, 4, 3]
//[9, 4, 7, 8, 7]

// Answer [3,7,5]
use std::collections::HashMap;
fn exception(nums1:Vec<i32>,nums2:Vec<i32>,nums3:Vec<i32>) -> Vec<i32>{
    let nums = [nums1,nums2,nums3];
    let mut hms:Vec<HashMap<i32,i32>> = vec![HashMap::new();3];
    let mut hm :HashMap<i32,i32> = HashMap::new();
    let mut vec = vec![];
    for i in 0..nums.len(){
        for n in nums[i].clone().into_iter(){
            if !hms[i].contains_key(&n){
                hms[i].insert(n, 1);
            }
        }
    }
    for hs in hms.into_iter(){
        for (key,_) in hs.into_iter(){
            *hm.entry(key).or_insert(0) += 1;
        }
    }

    for (key,val) in hm.into_iter(){
        if val < nums.len() as i32{
            vec.push(key);
        }
    }
    return vec;
}
fn main() {
    let nums2 = vec![4, 9, 5, 8, 9, 4];
    let nums3 = vec![9, 4, 9, 8, 3, 4, 3];
    let nums1 = vec![9, 4, 7, 8, 7];

    let vec = exception(nums1, nums2, nums3);
    println!("{:?}",vec);
}
