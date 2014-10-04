use roost::{Graph, SparseGraph, NodeIndex, EdgeIndex};
use std::collections::{HashSet, RingBuf, Deque};
use std::hash::Hash;
use std::iter::Iterator;

// struct DepthFirstVisit<'a, V:Clone+Eq, E, G:'a + Graph<V, E>>{
//     gp: &'a G,
//     next_nodes: Vec<NodeIndex>,
//     visited: HashSet<NodeIndex>,
// }

struct BreadthFirstVisit<'a, V:Clone+Eq, E, G:'a + Graph<V, E>>{
    gp: &'a G,
    next_nodes: RingBuf<NodeIndex>,
    visited: HashSet<NodeIndex>,
}

// pub fn depth_first_visit<'a, V:Clone+Eq, E, G: Graph<V, E>>(graph: &'a G, root: N)->DepthFirstVisit<'a, V, E, G>{
//     let mut visit:HashSet<NodeIndex> = HashSet::new();
//     visit.insert(root.clone());
//     DepthFirstVisit{gp: graph, next_nodes: vec![root], visited: visit}
// }

pub trait Searchable<V:Clone+Eq,E>:Graph<V,E> {
    fn breadth_first_visit(&self, root: &V)->BreadthFirstVisit<V, E, Self>{
        let mut visit:HashSet<NodeIndex> = HashSet::new();
        let mut queue = RingBuf::new();
        match self.index_of(root){
            Some(x) => {
                visit.insert(x);
                queue.push(x);
            },
            None    => {}
        }
        BreadthFirstVisit{gp: self, next_nodes: queue, visited: visit}
    }
}

impl <V:Clone+Eq, E:Clone> Searchable<V,E> for SparseGraph<V,E>{}

impl<'a, N:Clone+Eq, E, G: Graph<N, E>> Iterator<NodeIndex> for BreadthFirstVisit<'a, N, E, G>{
    fn next(&mut self) -> Option<NodeIndex>{
        let curr = self.next_nodes.pop_front();
        match curr {
            Some(node) => {
                let neighbors = self.gp.neighbors(node);
                for n in neighbors.into_iter(){
                    if !self.visited.contains(&n){
                        self.visited.insert(n);
                        self.next_nodes.push(n);
                    }
                }
            },
            None => {}
        }
        curr
    }
}
// pub fn breadth_first_visit<'a, N:Clone+Eq, E, G: Graph<N, E>>(graph: &'a G, root: N)->BreadthFirstVisit<'a, N, E, G>{
//     let mut visit:HashSet<NodeIndex> = HashSet::new();
//     let root_idx = graph.index_of(root);
//     visit.insert(root_idx);
//     let mut queue = RingBuf::new();
//     queue.push(root_idx);
//     BreadthFirstVisit{gp: graph, next_nodes: queue, visited: visit}
// }

// impl<'a, N:Clone+Hash+Eq, G: Graph<N>> Iterator<N> for DepthFirstVisit<'a, N, G>{
//     fn next(&mut self) -> Option<N>{
//         let curr = self.next_nodes.pop();
//         match curr {
//             Some(ref node) => {
//                 let neighbors = self.gp.neighbors(node.clone());
//                 for n in neighbors.into_iter(){
//                     if !self.visited.contains(&n){
//                         self.visited.insert(n.clone());
//                         self.next_nodes.push(n.clone());
//                     }
//                 }
//             },
//             None => {}
//         }
//         curr
//     }
// }


#[test]
fn breadth_first_search(){
    let mut graph:SparseGraph<int, int> = SparseGraph::new();
    graph.add_edge(1i, 2i, 1);
    graph.add_edge(1i, 3i, 2);
    graph.add_edge(1i, 6i, 1);
    graph.add_edge(2i, 5i, 4);
    graph.add_edge(3i, 6i, 6);
    graph.add_edge(3i, 7i, 7);
    println!("Breadth first visit:")
    for idx in graph.breadth_first_visit(&1i){
        println!("Visited: {}", graph.node_of(idx));
    }
}
