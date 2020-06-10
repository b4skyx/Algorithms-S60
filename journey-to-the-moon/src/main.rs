use std::clone::Clone;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;
struct Graph<T: Hash + Eq + Clone> {
    nodes: HashMap<T, Node<T>>,
    bidirectional: bool,
}
impl<T: Hash + Eq + Clone> Graph<T> {
    fn new(bidirectional: bool) -> Self {
        Graph {
            nodes: HashMap::new(),
            bidirectional,
        }
    }
    fn add_node(&mut self, identifier: T) {
        self.nodes.insert(identifier.clone(), Node::new(identifier));
    }
    fn add_pair(&mut self, a: T, b: T) {
        if let Some(node) = self.nodes.get_mut(&a) {
            node.add_adj(b.clone());
        } else {
            self.add_node(a.clone());
            self.add_pair(a.clone(), b.clone());
        }
        if self.bidirectional == true {
            if let Some(node) = self.nodes.get_mut(&b) {
                node.add_adj(a);
            } else {
                self.add_node(b.clone());
                self.add_pair(b, a);
            }
        }
    }
}

struct Node<T: Hash + Eq> {
    identifier: T,
    adj_list: HashSet<T>,
    //paylaoad
}
impl<T: Hash + Eq> Node<T> {
    fn new(identifier: T) -> Node<T> {
        Node {
            identifier: identifier,
            adj_list: HashSet::new(),
        }
    }
    fn add_adj(&mut self, identifier: T) {
        self.adj_list.insert(identifier);
    }
}
fn journey_calc() {
    let n = read_input();
    let mut graph: Graph<u32> = Graph::new(true);
    let mut res;
    for _ in 0..n[1] {
        let x = read_input();
        graph.add_pair(x[0], x[1]);
    }
    res = ((n[0] as u64 * (n[0] as u64 - 1)) / 2) as u64;
    let mut visited = HashSet::new();
    for k in graph.nodes.values() {
        let mut bfs = vec![k.identifier];
        let mut w = 1u64;
        if !visited.contains(&k.identifier) {
            visited.insert(k.identifier);
            while !bfs.is_empty() {
                let t = bfs.remove(0);
                for i in &graph.nodes[&t].adj_list {
                    if !visited.contains(&i) {
                        visited.insert(*i);
                        // println!("Pushed {}", i);
                        bfs.push(*i);
                        w += 1;
                    }
                }
            }
            // println!("{:?}", visited);
        }
        // println!(" W: {}", w * (w - 1) / 2);
        res -= (w * (w - 1)) / 2;
    }
    // println!("Visited: {:?}", visited);
    println!("{}", res);
}

fn read_input() -> Vec<u32> {
    let mut ipt = String::new();
    io::stdin().read_line(&mut ipt).expect("Err");
    ipt.split_ascii_whitespace()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn main() {
    journey_calc();
}
