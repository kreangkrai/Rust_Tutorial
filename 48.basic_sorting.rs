//Basic Sorting

/*
    first : Descending of Salary
    second : Ascending of Name
    
    ["C","ACF",50000]
    ["D","ACF",10000]
    ["E","ACF",20000]
    ["F","ACF",50000]
    ["A","ACF",50000]
    ["B","ACF",10000]

    Answer
    ["A","ACF",50000]
    ["C","ACF",50000]
    ["F","ACF",50000]  
    ["E","ACF",20000]
    ["B","ACF",10000]
    ["D","ACF",10000]
 */
#[derive(Debug)]
#[allow(dead_code)]
struct Employee {
    name :&'static str,
    department : &'static str,
    salary : u32,
}
impl Employee {
    fn new(name:&'static str,department:&'static str,salary:u32) -> Self {
        Employee { name: name, department: department, salary: salary }
    }
}
fn main() {  
    let mut emps:Vec<Employee> = vec![];
    emps.push(Employee::new("C","ACF",50000 ));
    emps.push(Employee::new("D","ACF",10000 ));
    emps.push(Employee::new("E","ACF",20000 ));
    emps.push(Employee::new("F","ACF",50000 ));
    emps.push(Employee::new("A","ACF",50000 ));
    emps.push(Employee::new("B","ACF",10000 ));

    emps.sort_by(|a,b| b.salary.cmp(&a.salary).then(a.name.cmp(&b.name)));
    for emp in emps.into_iter() {
        println!("{:?}",emp);
    }   
}
