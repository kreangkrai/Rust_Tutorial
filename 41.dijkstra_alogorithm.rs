use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Vertex {
    name: &'static str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Edge {
    weight: u32,
    from: Vertex,
    to: Vertex,
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

#[allow(dead_code)]
impl Graph {
    fn new() -> Graph {
        Graph {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_vertex(&mut self, name: &'static str) {
        self.vertices.push(Vertex { name });
    }

    fn get_vertex(&self, name: &str) -> &Vertex {
        let vertex = self.vertices.iter().find(|x| x.name == name).unwrap();
        vertex
    }
    fn add_edge(&mut self, from: &str, to: &str, weight: u32) {
        let from = self.get_vertex(from);
        let to = self.get_vertex(to);
        self.edges.push(Edge {
            weight,
            from: *from,
            to: *to,
        });
    }
    fn get_edge(&self, name: &str) -> Vec<&Edge> {
        let vertex = self.get_vertex(name);
        self.edges.iter().filter(|edge| edge.from == *vertex).collect()
    }
    fn get_adges(&self) -> Vec<&Edge> {
        self.edges.iter().collect()
    }
    fn get_labels(&self) -> Vec<&str> {
        self.vertices.iter().map(|m| m.name).collect::<Vec<&str>>()
    }
    fn crate_adj_matrix(&self) -> Vec<Vec<u32>> {
        let edges = self.edges.clone();
        let labels = self.get_labels();
        let mut adj: Vec<Vec<u32>> = vec![vec![0u32; labels.len()]; labels.len()];
        for edge in edges.into_iter() {
            let row = labels
                .clone()
                .into_iter()
                .position(|p| p == edge.from.name)
                .unwrap();
            let col = labels
                .clone()
                .into_iter()
                .position(|p| p == edge.to.name)
                .unwrap();
            adj[row][col] = edge.weight;
            adj[col][row] = edge.weight;
        }
        adj
    }

    fn minidist(distance: Vec<u32>, tset: Vec<bool>) -> usize {
        let mut minimum = u32::MAX;
        let mut index: usize = 0;
        for (i, k) in distance.into_iter().enumerate() {
            if !tset[i] && k <= minimum {
                minimum = k;
                index = i;
            }
        }
        index
    }
    fn dijkstra(graph:Vec<Vec<u32>>, src: usize, dest: usize) -> Option<Vec<u32>> {
        let length = graph.len();
        let mut distance = vec![u32::MAX; length];
        let mut used = vec![false; length];
        let mut prev = vec![None; length];
        distance[src] = 0;
        for _ in 0..length - 1 {
            let min_graph = Graph::minidist(distance.clone(), used.clone());
            used[min_graph] = true;
            for i in 0..length {
                if graph[min_graph][i] > 0 {
                    let shortest_to_mingraph = distance[min_graph];
                    let distance_to_nextgraph = graph[min_graph][i];
                    let total_distance = shortest_to_mingraph + distance_to_nextgraph;
                    if total_distance < distance[i] {
                        distance[i] = total_distance;
                        prev[i] = Some(min_graph);
                    }
                }
            }
        }
        if distance[dest] == u32::MAX {
            return None;
        }
        let mut path: LinkedList<u32> = LinkedList::new();
        let mut current_graph: Option<usize> = Some(dest);

        while let Some(p) = current_graph {
            path.push_front(p as u32);
            current_graph = prev[p];
        }
        Some(path.into_iter().collect::<Vec<u32>>())
    }
    fn shortest_path(&self,src: &str, dest: &str) {
        let graph = self.crate_adj_matrix();
        let labels = self.get_labels();

        let source = labels.clone().into_iter().position(|p| p == src).unwrap() as usize;
        let destination = labels.clone().into_iter().position(|p| p == dest).unwrap() as usize;

        print!("Shortest Path of [{} -> {}] is : ", src, dest);
        let paths = Graph::dijkstra(graph.clone(),source, destination);
        if let Some(p) = paths {
            let mut path_length = 0;
            for i in 0..p.len() - 1 {
                let length: u32 = graph[p[i] as usize][p[i + 1] as usize];
                path_length += length;
                print!("{} [{}] -> ", labels[p[i] as usize], length);
            }
            println!("{} (Distance {})", labels[destination], path_length);
        } else {
            println!("No Path");
        }
    }
}
fn main() {
    let mut graph = Graph::new();
    graph.add_vertex("A");
    graph.add_vertex("B");
    graph.add_vertex("C");
    graph.add_vertex("D");
    graph.add_vertex("E");
    graph.add_vertex("F");
    graph.add_vertex("G");
    graph.add_vertex("H");
    graph.add_vertex("I");
    graph.add_vertex("J");
    graph.add_vertex("K");
    graph.add_vertex("L");

    graph.add_edge("A", "B", 3);
    graph.add_edge("A", "C", 2);
    graph.add_edge("B", "C", 5);
    graph.add_edge("B", "D", 2);
    graph.add_edge("B", "G", 7);
    graph.add_edge("C", "E", 2);
    graph.add_edge("C", "F", 9);
    graph.add_edge("D", "E", 8);
    graph.add_edge("D", "F", 1);
    graph.add_edge("E", "G", 3);
    graph.add_edge("F", "G", 6);
    graph.add_edge("F", "H", 7);
    graph.add_edge("F", "K", 8);
    graph.add_edge("G", "I", 6);
    graph.add_edge("G", "J", 9);
    graph.add_edge("H", "I", 7);
    graph.add_edge("H", "J", 2);
    graph.add_edge("I", "K", 4);
    graph.add_edge("J", "K", 6);
    graph.add_edge("J", "L", 4);
    graph.add_edge("K", "L", 5);

    graph.shortest_path("A", "L");

}
