//Binary Tree
/*
                   5
                /    \
              3        7
            /  \      /  \
           2    4    6    8
          /                \
         1                  9
*/
use std::cmp::Ordering;
use std::cmp::Ord;
use std::collections::{HashMap, VecDeque};
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}
#[allow(non_shorthand_field_patterns)]
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    fn left(mut self, node: TreeNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    fn right(mut self, node: TreeNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }
    fn bfs(&self) -> Vec<(usize, Vec<i32>)> {
        let mut res: Vec<(usize, i32)> = vec![];
        let mut q = VecDeque::new();
        q.push_back((0, self));
        while q.len() > 0 {
            let (level, node) = q.pop_front().unwrap();

            res.push((level, node.val));

            if let Some(ref l) = node.left {
                q.push_back((level + 1, l));
            }
            if let Some(ref r) = node.right {
                q.push_back((level + 1, r));
            }
        }
        let mut hm: HashMap<usize, Vec<i32>> = HashMap::new();
        for r in res.into_iter() {
            hm.entry(r.0).or_insert(Vec::new()).push(r.1);
        }

        let mut hm_vec: Vec<(usize, Vec<i32>)> = hm.into_iter().collect();
        hm_vec.sort_by(|a, b| a.0.cmp(&b.0));
        hm_vec
    }
    fn traversal_inorder(&self, node: Option<Box<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            self.traversal_inorder(n.left.clone(), res);
            res.push(n.val);
            self.traversal_inorder(n.right.clone(), res);
        }
    }
    fn inorder(&self) -> Vec<i32> {
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_inorder(node, &mut res);
        res
    }
    fn traversal_preorder(&self, node: Option<Box<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            res.push(n.val);
            self.traversal_preorder(n.left.clone(), res);
            self.traversal_preorder(n.right.clone(), res);
        }
    }
    fn preorder(&self) -> Vec<i32> {
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_preorder(node, &mut res);
        res
    }
    fn traversal_postorder(&self, node: Option<Box<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(n) = node {
            self.traversal_postorder(n.left.clone(), res);
            self.traversal_postorder(n.right.clone(), res);
            res.push(n.val);
        }
    }
    fn postorder(&self) -> Vec<i32> {
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_postorder(node, &mut res);
        res
    }
    fn traversal_leaf(&self, node: Option<Box<TreeNode>>, res: &mut Vec<i32>) {
        if let Some(curr_node) = node {
            if curr_node.left.is_none() && curr_node.right.is_none() {
                res.push(curr_node.val);
                return;
            }
            self.traversal_leaf(curr_node.left, res);
            self.traversal_leaf(curr_node.right, res);
        }
    }
    fn leaf(&self) -> Vec<i32> {
        let mut res = vec![];
        let node = Some(Box::new(self.clone()));
        self.traversal_leaf(node, &mut res);
        res
    }
    fn traversal_search(&self, node: Option<Box<TreeNode>>, value: i32) {
        if let Some(curr_node) = node.clone() {
            match curr_node.val.cmp(&value) {
                Ordering::Equal => {
                    println!(" => Found Value [{}]", value);
                }
                Ordering::Less => {
                    if curr_node.right.is_some() {
                        print!("[{}] -> Right ", curr_node.val);
                        self.traversal_search(curr_node.right.clone(), value);
                    } else {
                        print!("[{}] -> Right ", curr_node.val);
                        self.traversal_search(None, value);
                    }
                }
                Ordering::Greater => {
                    if curr_node.left.is_some() {
                        print!("[{}] -> Left ", curr_node.val);
                        self.traversal_search(curr_node.left.clone(), value);
                    } else {
                        print!("[{}] -> Left ", curr_node.val);
                        self.traversal_search(None, value);
                    }
                }
            }
        } else {
            println!(" => Not Found [{}]", value);
        }
    }
    fn search(&self, value: i32) {
        let node = Some(Box::new(self.clone()));
        self.traversal_search(node, value);
    }
    fn traversal_rigth_side(&self) -> Vec<(usize, Vec<i32>)> {
        let mut res: Vec<(usize, i32)> = vec![];
        let mut q = VecDeque::new();
        q.push_back((0, self));
        while q.len() > 0 {
            let bound = q.len() - 1;
            for i in 0..=bound {
                let (level, node) = q.pop_front().unwrap();
                if i == bound {
                    res.push((level, node.val));
                }
                if let Some(ref l) = node.left {
                    q.push_back((level + 1, l));
                }
                if let Some(ref r) = node.right {
                    q.push_back((level + 1, r));
                }
            }
        }
        let mut hm: HashMap<usize, Vec<i32>> = HashMap::new();
        for r in res.into_iter() {
            hm.entry(r.0).or_insert(Vec::new()).push(r.1);
        }

        let mut hm_vec: Vec<(usize, Vec<i32>)> = hm.into_iter().collect();
        hm_vec.sort_by(|a, b| a.0.cmp(&b.0));
        hm_vec
    }
    fn traversal_left_side(&self) -> Vec<(usize, Vec<i32>)> {
        let mut res: Vec<(usize, i32)> = vec![];
        let mut q = VecDeque::new();
        q.push_back((0, self));
        while q.len() > 0 {
            let bound = q.len() - 1;
            for i in 0..=bound {
                let (level, node) = q.pop_front().unwrap();
                if i == 0 {
                    res.push((level, node.val));
                }
                if let Some(ref l) = node.left {
                    q.push_back((level + 1, l));
                }
                if let Some(ref r) = node.right {
                    q.push_back((level + 1, r));
                }
            }
        }
        let mut hm: HashMap<usize, Vec<i32>> = HashMap::new();
        for r in res.into_iter() {
            hm.entry(r.0).or_insert(Vec::new()).push(r.1);
        }

        let mut hm_vec: Vec<(usize, Vec<i32>)> = hm.into_iter().collect();
        hm_vec.sort_by(|a, b| a.0.cmp(&b.0));
        hm_vec
    }
    fn traversal_parent(&self,mut node: Option<Box<TreeNode>>,value: i32,) -> Option<Box<TreeNode>> {
        if let Some(curr_node) = node.clone() {
            match curr_node.val.cmp(&value) {
                Ordering::Equal => {
                    return Some(curr_node);
                }
                Ordering::Less => {
                    if curr_node.right.is_some() {
                        node = self.traversal_parent(curr_node.right.clone(), value);
                    } else {
                        node = self.traversal_parent(None, value);
                    }
                }
                Ordering::Greater => {
                    if curr_node.left.is_some() {
                        node = self.traversal_parent(curr_node.left.clone(), value);
                    } else {
                        node = self.traversal_parent(None, value);
                    }
                }
            }
        } else {
            return None;
        }
        node
    }
    fn child_of_parent(&self, value: i32) -> Vec<(usize, Vec<i32>)> {
        let node = Some(Box::new(self.clone()));
        let child = self.traversal_parent(node, value);

        let mut res: Vec<(usize, i32)> = vec![];
        let mut q = VecDeque::new();
        if let Some(c) = &child.clone() {
            q.push_back((0, c));
            while q.len() > 0 {
                let (level, c) = q.pop_front().unwrap();

                res.push((level, c.val));

                if let Some(ref l) = c.left {
                    q.push_back((level + 1, l));
                }
                if let Some(ref r) = c.right {
                    q.push_back((level + 1, r));
                }
            }
            let mut hm: HashMap<usize, Vec<i32>> = HashMap::new();
            for r in res.into_iter() {
                hm.entry(r.0).or_insert(Vec::new()).push(r.1);
            }

            let mut hm_vec: Vec<(usize, Vec<i32>)> = hm.into_iter().collect();
            hm_vec.sort_by(|a, b| a.0.cmp(&b.0));
            hm_vec
        }else{
            vec![]
        }
    }
}
fn main() {
    let root = TreeNode::new(5)
        .left(
            TreeNode::new(3)
                .left(TreeNode::new(2).left(TreeNode::new(1)))
                .right(TreeNode::new(4)),
        )
        .right(
            TreeNode::new(7)
                .left(TreeNode::new(6))
                .right(TreeNode::new(8).right(TreeNode::new(9))),
        );

    let res = root.bfs();
    println!("=============== BFS ==================");
    for (key, value) in res.into_iter() {
        print!("Level {:?} => ", key);
        for v in value.into_iter() {
            print!("{:?} ", v);
        }
        println!();
    }
    println!("=============== DFS ==================");
    let inorder = root.inorder();
    println!("In   Order {:?}", inorder);

    let preorder = root.preorder();
    println!("Pre  Order {:?}", preorder);

    let postoerder = root.postorder();
    println!("Post Order {:?}", postoerder);

    println!("=============== Leaf =================");
    let leaf = root.leaf();
    println!("Leaf  Node {:?}", leaf);

    println!("============== Search ================");
    root.search(-4);

    println!("============= Right Side =============");
    let right_side = root.traversal_rigth_side();
    for (key, value) in right_side.into_iter() {
        print!("Level {:?} => ", key);
        for v in value.into_iter() {
            print!("{:?} ", v);
        }
        println!();
    }

    println!("============= Left Side ==============");
    let left_side = root.traversal_left_side();
    for (key, value) in left_side.into_iter() {
        print!("Level {:?} => ", key);
        for v in value.into_iter() {
            print!("{:?} ", v);
        }
        println!();
    }

    println!("========== Child Of Parent ===========");
    let child = root.child_of_parent(8);
    if child.len() > 0 {
        for (key, value) in child.into_iter() {
            print!("Level {:?} => ", key);
            for v in value.into_iter() {
                print!("{:?} ", v);
            }
            println!();
        }
    }else{
        println!("Empty");
    }
}
