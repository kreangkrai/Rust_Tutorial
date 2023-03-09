//Garph Traversal : BFS Algorithm With Rust Programing Language
#[derive(Debug,Clone)]
struct Queue<T>{
    queue:Vec<T>,
}
#[allow(dead_code)]
impl <T> Queue<T> {
    fn new()->Self{
        Queue { queue: Vec::new() }
    }
    fn enqueue(&mut self,item:T){
        self.queue.push(item);
    }
    fn dequeue(&mut self)->T{
        self.queue.remove(0)
    }
    fn length(&self)->usize{
        self.queue.len()
    }
    fn isempty(&self)-> bool{
        self.queue.is_empty()
    }
    fn peek(&self)->Option<&T>{
        self.queue.first()
    }
    fn last(&self)->Option<&T>{
        self.queue.last()
    }
    fn front(&self)->Option<&T>{
        self.queue.first()
    }
}
#[derive(Debug,Clone)]
struct Vertex{
    name: &'static str,
    visited:bool,
    index:u32,
}
#[derive(Debug,Clone)]
struct Edge{
    from:Vertex,
    to:Vertex,
}
struct Graph{
    vertices : Vec<Vertex>,
    edges :Vec<Edge>,
    queue_ans:Queue<Vertex>,
    label_ans:Vec<&'static str>,
}
#[allow(dead_code)]
impl Graph {
    fn new() ->Self{
        Graph { vertices: vec![], edges: vec![], queue_ans: Queue::new(), label_ans: vec![] }
    }
    fn insert_vertex(&mut self,name:&'static str,index:u32){
        let v = Vertex{
            name:name,
            visited:false,
            index:index,
        };
        self.vertices.push(v);
    }
    fn get_vertex(&self,name:&'static str)-> Option<Vertex>{
        for v in self.vertices.clone().into_iter(){
            if v.name == name{
                return Some(v);
            }
        }
        None
    }
    fn insert_edge(&mut self,from:&'static str,to :&'static str){
        let f = self.get_vertex(from);
        let t = self.get_vertex(to);
        let edge = Edge{
            from:f.unwrap(),
            to:t.unwrap(),
        };
        self.edges.push(edge);
    }
    fn get_labels(&self)->Vec<&'static str>{
        let mut labels:Vec<&'static str> = vec![];

        for v in self.vertices.clone().into_iter(){
            labels.push(v.name);
        }
        labels
    }
    fn create_adj_matrix(&self)->Vec<Vec<u32>>{
        let labels = self.get_labels();
        let mut adj:Vec<Vec<u32>> = vec![vec![0;labels.len()];labels.len()];
        let mut row = 0;
        let mut col = 0;

        for edge in self.edges.clone().into_iter(){
            for (i,label) in labels.clone().into_iter().enumerate(){
                if label == edge.from.name{
                    row = i;
                }
                if label == edge.to.name{
                    col = i;
                }
            }
            adj[row][col] = 1;
            adj[col][row] = 1;
        }
        adj
    }
    fn get_unvisited_vertex(&self,vertex_index:u32) -> Option<u32>{
        let adj:Vec<Vec<u32>> = self.create_adj_matrix();
        for v in self.vertices.clone().into_iter(){
            if adj[vertex_index as usize][v.index as usize] == 1 && v.visited == false{
                return Some(v.index);
            }
        }
        return None;
    }
    fn display_vertex(&self){
        for label in self.label_ans.clone().into_iter(){
            print!("{} ",label);
        }
    }
    fn bfs(&mut self){
        self.vertices[0].visited = true;
        self.queue_ans.enqueue(self.vertices[0].to_owned());

        while !self.queue_ans.isempty() {
            let unvisited_vertex = self.get_unvisited_vertex(self.queue_ans.peek().unwrap().index);
            match unvisited_vertex {
                Some(u) => {
                    self.vertices[u as usize].visited = true;
                    self.queue_ans.enqueue(self.vertices[u as usize].to_owned());
                },
                None =>{
                    self.label_ans.push(self.queue_ans.front().unwrap().name);
                    self.queue_ans.dequeue();
                }
            }
        }
        self.display_vertex();
    }
}
fn main(){
    let mut graph = Graph::new();
    graph.insert_vertex("A", 0);
    graph.insert_vertex("B", 1);
    graph.insert_vertex("C", 2);
    graph.insert_vertex("D", 3);
    graph.insert_vertex("E", 4);
    graph.insert_vertex("F", 5);
    graph.insert_vertex("G", 6);
    graph.insert_vertex("H", 7);
    graph.insert_vertex("I", 8);
    graph.insert_vertex("J", 9);

    graph.insert_edge("A", "B");
    graph.insert_edge("A", "C");
    graph.insert_edge("A", "D");
    graph.insert_edge("B", "E");
    graph.insert_edge("C", "F");
    graph.insert_edge("D", "G");
    graph.insert_edge("E", "H");
    graph.insert_edge("F", "J");
    graph.insert_edge("G", "I");
    graph.insert_edge("H", "J");
    graph.insert_edge("I", "J");

    graph.bfs();
}
