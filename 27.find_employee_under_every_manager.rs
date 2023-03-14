/*
Find Employee Under every manager With Rust Programing Language

A <--> C
B <--> C
C <--> F
D <--> E
E <--> F
F <--> F
G <--> A
H <--> F
I <--> A

===== Answer =====

E => [D]
A => [G,I]
C => [B,A,G,I]
F => [C,E,H,A,B,G,I,D]

                    F
                 /  |  \
                /   |   \
               C    E    H
             /   \    \
            /     \    \
           A       B    D
         /   \
        /     \
       G       I

 */

use std::collections::HashMap;
struct Employee{
    employee:HashMap<&'static str,&'static str>,
}
impl Employee {
    fn new(emp:HashMap<&'static str,&'static str>)-> Self{
        Employee{
            employee:emp
        }
    }
    fn calculate(&self) -> HashMap<&'static str,Vec<&'static str>>{
        let mut manager : HashMap<&'static str,Vec<&'static str>> = HashMap::new();
        for (key,val) in self.employee.clone().into_iter(){
            if key != val {
                manager.entry(val).or_insert(Vec::new()).push(key);
            }
        }
        for (key,vals) in manager.clone().into_iter(){
            for emp in vals.clone().into_iter(){
                let emps = manager
                            .clone()
                            .into_iter()
                            .find(|(k,_)| k == &emp)
                            .map(|(_,v)| v);
                match emps {
                    Some(e) => {
                        for n in e.iter() {
                            manager.entry(key).or_insert(vals.clone()).push(n);
                        }
                    },
                    None => {
                        manager.entry(key).or_insert(vals.clone());
                    }
                }
            }
        }
        return manager;
    }
}
fn main() {
    let mut emp: HashMap<&'static str, &'static str> = HashMap::new();
    emp.insert("A", "C");
    emp.insert("B", "C");
    emp.insert("C", "F");
    emp.insert("D", "E");
    emp.insert("E", "F");
    emp.insert("F", "F");
    emp.insert("G", "A");
    emp.insert("H", "F");
    emp.insert("I", "A");

    let emps : Employee = Employee::new(emp);
    let manager = emps.calculate();

    for m in manager {
        println!("{:?} => {:?}",m.0,m.1);
    }
}
