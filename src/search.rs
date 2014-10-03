use roost::{Graph, SparseGraph, Edge};
use std::collections::{HashSet, RingBuf, Deque};
use std::hash::Hash;
use std::iter::Iterator;

struct DepthFirstVisit<'a, N:Clone+Hash+Eq, G:'a + Graph<N>>{
    gp: &'a G,
    next_nodes: Vec<N>,
    visited: HashSet<N>,
}

struct BreadthFirstVisit<'a, N:Clone+Hash+Eq, G:'a + Graph<N>>{
    gp: &'a G,
    next_nodes: RingBuf<N>,
    visited: HashSet<N>,
}

pub fn depth_first_visit<'a, N:Clone+Hash+Eq, G: Graph<N>>(graph: &'a G, root: N)->DepthFirstVisit<'a, N, G>{
    let mut visit:HashSet<N> = HashSet::new();
    visit.insert(root.clone());
    DepthFirstVisit{gp: graph, next_nodes: vec![root], visited: visit}
}

pub fn breadth_first_visit<'a, N:Clone+Hash+Eq, G: Graph<N>>(graph: &'a G, root: N)->BreadthFirstVisit<'a, N, G>{
    let mut visit:HashSet<N> = HashSet::new();
    visit.insert(root.clone());
    let mut queue = RingBuf::new();
    queue.push(root);
    BreadthFirstVisit{gp: graph, next_nodes: queue, visited: visit}
}

impl<'a, N:Clone+Hash+Eq, G: Graph<N>> Iterator<N> for DepthFirstVisit<'a, N, G>{
    fn next(&mut self) -> Option<N>{
        let curr = self.next_nodes.pop();
        match curr {
            Some(ref node) => {
                let neighbors = self.gp.neighbors(node.clone());
                for n in neighbors.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n.clone());
                        self.next_nodes.push(n.clone());
                    }
                }
            },
            None => {}
        }
        curr
    }
}

impl<'a, N:Clone+Hash+Eq, G: Graph<N>> Iterator<N> for BreadthFirstVisit<'a, N, G>{
    fn next(&mut self) -> Option<N>{
        let curr = self.next_nodes.pop_front();
        match curr {
            Some(ref node) => {
                let neighbors = self.gp.neighbors(node.clone());
                for n in neighbors.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n.clone());
                        self.next_nodes.push_back(n.clone());
                    }
                }
            },
            None => {}
        }
        curr
    }
}

#[test]
fn depth_first_search(){
    let edges = vec![
        Edge{from: 1i, to: 2, weight: 1.0},
        Edge{from: 1i, to: 3, weight: 2.5},
        Edge{from: 2i, to: 4, weight: 1.0},
        Edge{from: 2i, to: 5, weight: 1.0},
        Edge{from: 3i, to: 6, weight: 1.0},
        Edge{from: 3i, to: 7, weight: 1.0},
    ];
    let gp:SparseGraph<int> = edges.into_iter().collect();
    println!("Depth first visit:")
    for node in depth_first_visit(&gp, 1){
        println!("Visited: {}", node);
    }
    println!("Breadth first visit:")
    for node in breadth_first_visit(&gp, 1){
        println!("Visited: {}", node);
    }
}
