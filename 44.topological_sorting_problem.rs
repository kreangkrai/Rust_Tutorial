//Topological Sorting Problem With Rust Programing Language
#[derive(Debug, Clone)]
struct Stack<T> {
    stack: Vec<T>,
}
#[allow(dead_code)]
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn length(&self) -> usize {
        self.stack.len()
    }
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
    fn read(&mut self) -> Vec<T> {
        let mut jobs = vec![];
        while !self.stack.is_empty() {
            let j = self.pop().unwrap();
            jobs.push(j);
        }
        jobs
    }
}
#[derive(Debug,Clone, Copy)]
struct JOB {
    job: &'static str,
    id: u32,
}

struct Graph {
    adj_job: Vec<Vec<JOB>>,
    stack: Stack<JOB>,
}
impl Graph {
    fn new() -> Self {
        Graph {
            adj_job: Vec::new(),
            stack: Stack::new(),
        }
    }
    fn create_vertex(&mut self, v: JOB) {
        self.adj_job.push(vec![v]);
    }
    fn add_edge(&mut self, v: JOB, w: JOB) {
        self.adj_job[v.id as usize].push(w);
    }
    fn get_edge(&self) -> Vec<(JOB, JOB)> {
        let mut job: Vec<(JOB, JOB)> = vec![];
        for ad in self.adj_job.iter() {
            for i in 1..ad.len() {
                job.push((
                    JOB {
                        id: ad[0].id,
                        job: ad[0].job,
                    },
                    JOB {
                        id: ad[i].id,
                        job: ad[i].job,
                    },
                ));
            }
        }
        job
    }
    fn topological_sort_util(&self, v: JOB, visited: &mut Vec<bool>, stack: &mut Stack<JOB>) -> Stack<JOB> {
        visited[v.id as usize] = true;
        for vertex in self.adj_job[v.id as usize].iter() {
            if !visited[vertex.id as usize] {
                self.topological_sort_util(vertex.to_owned(), visited,stack);
            }
        }
        stack.push(v);
        stack.to_owned()
    }
    fn topological_sort(&mut self) -> Stack<JOB> {
        let mut visited = vec![false; self.adj_job.len()];
        for (i, adj) in self.adj_job.iter().enumerate() {
            if visited[i] == false {
                self.stack =  self.topological_sort_util(
                    adj.first().unwrap().to_owned(),
                    &mut visited,
                    &mut self.stack.to_owned()
                )
            }
        }
        self.stack.to_owned()
    }
    fn check_valid_edge(&self, j1: JOB, j2: JOB, jobs: Vec<JOB>) -> bool {
        let mut b = false;
        for j in jobs.iter() {
            if j1.job == j.job {
                b = true;
                break;
            } else if j2.job == j.job {
                break;
            }
        }
        b
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}
#[allow(unused_assignments)]
fn main() {
    let mut g = Graph::new();

    let j0 = JOB { id: 0, job: "Job0" };
    let j1 = JOB { id: 1, job: "Job1" };
    let j2 = JOB { id: 2, job: "Job2" };
    let j3 = JOB { id: 3, job: "Job3" };
    let j4 = JOB { id: 4, job: "Job4" };
    let j5 = JOB { id: 5, job: "Job5" };
    let j6 = JOB { id: 6, job: "Job6" };
    let j7 = JOB { id: 7, job: "Job7" };
    let j8 = JOB { id: 8, job: "Job8" };
    let j9 = JOB { id: 9, job: "Job9" };

    g.create_vertex(j0);
    g.create_vertex(j1);
    g.create_vertex(j2);
    g.create_vertex(j3);
    g.create_vertex(j4);
    g.create_vertex(j5);
    g.create_vertex(j6);
    g.create_vertex(j7);
    g.create_vertex(j8);
    g.create_vertex(j9);

    g.add_edge(j0, j1);
    g.add_edge(j0, j5);
    g.add_edge(j0, j6);
    g.add_edge(j1, j2);
    g.add_edge(j2, j3);
    g.add_edge(j2, j5);
    g.add_edge(j2, j7);
    g.add_edge(j3, j4);
    g.add_edge(j3, j7);
    g.add_edge(j4, j9);
    g.add_edge(j5, j6);
    g.add_edge(j6, j7);
    g.add_edge(j7, j8);
    g.add_edge(j8, j9);
    g.add_edge(j9, j2);

    let mut stack = g.topological_sort();
    let mut str:Vec<&str> = vec![];
    let vertexs = stack.read();
    for vertex in vertexs.iter(){
        str.push(vertex.job);
    }
    let join = str.join(" -> ");
    println!("{:?}",join);

    let mut job:Vec<(JOB,JOB)> = vec![];
    job = g.get_edge();

    for j in job.iter(){
        if !g.check_valid_edge(j.0, j.1, vertexs.clone()){
            println!("Remove Edge : {:?} -> {:?}",j.0.job,j.1.job);
        }
    }
}
