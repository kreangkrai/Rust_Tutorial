//Function Pass By Value and Pass By Reference

fn push_by_value(n:usize)->Vec<usize>{
    let mut vec = vec![];
    for i in 0..n{
        vec.push(i);
    }
    vec
}

fn push_by_ref(n:usize,res:&mut Vec<usize>){
    for i in res.len()..res.len() + n{
        res.push(i);
    }
}
#[allow(unused_assignments)]
fn main() {
    //function pass by value
    //push data 0 - 9
    let mut v:Vec<usize> = vec![];
    v = push_by_value(10);
    println!("{:?}",v);

    //function pass by reference
    //push data continue from pass by value 5 more times (10,11,12,13,14)
    push_by_ref(5,&mut v);
    println!("{:?}",v);
  }
