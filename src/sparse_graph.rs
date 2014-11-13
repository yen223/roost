use std::collections::HashMap;
use std::vec::Vec;
use graph::{Graph, Node, NodeIndex, EdgeIndex};
use graph::graph_error::NodeNotFound;
pub struct SparseGraph<V, E>
    where V: Clone,
          E: Clone,
{
    nodes: Vec<V>,
    edges: HashMap<EdgeIndex,E>,
    adj_list: Vec<Vec<NodeIndex>>,
}

impl<V, E> SparseGraph<V, E>
    where V: Eq+Clone,
          E: Clone,
{
    pub fn new()->SparseGraph<V, E>{
        let n:Vec<V> = Vec::new();
        let e:HashMap<EdgeIndex, E> = HashMap::new();
        let adjl:Vec<Vec<NodeIndex>> = Vec::new();
        SparseGraph{nodes: n, edges: e, adj_list: adjl}
    }

    fn add_node(&mut self, n: V){
        self.nodes.push(n);
        self.adj_list.push(Vec::new());
    }

    pub fn add_edge(&mut self, from: V, to: V, edge: E){
        let fi:NodeIndex = match self.node_to_index(&from).ok(){
            Some(x) => x,
            None    => {
                self.add_node(from);
                self.nodes.len()-1
            },
        };
        let ti:NodeIndex = match self.node_to_index(&to).ok(){
            Some(x) => x,
            None    => {
                self.add_node(to);
                self.nodes.len()-1
            }
        };
        self.edges.insert((fi, ti), edge);
        self.adj_list[fi].push(ti);
    }
}

impl<V, E> FromIterator<(V, V, E)> for SparseGraph<V, E>
    where V: Eq+Clone,
          E: Clone,
{
    fn from_iter<T: Iterator<(V, V, E)>>(mut iterator: T) -> SparseGraph<V, E>{
        let mut graph:SparseGraph<V, E> = SparseGraph::new();
        for (from, to, edge) in iterator{
            graph.add_edge(from, to, edge);
        }
        graph
    }
}

impl<V, E> Graph<V, E> for SparseGraph<V, E> 
    where V: Eq+Clone,
          E: Clone,
{
    fn insert_node(&mut self, node: V)->NodeIndex{
        self.add_node(node);
        return self.nodes.len()-1
    }

    fn node_to_index(&self, n: &V) -> Node {
        let nds = self.nodes.as_slice();
        for (idx, x) in nds.iter().enumerate(){
            if *x == *n {return Ok(idx);}
        }
        return Err(NodeNotFound);
    }

    fn index_to_node(&self, idx: NodeIndex) -> Option<V>{
        if idx > self.nodes.len()-1 {
            None
        } else {
            Some(self.nodes[idx].clone())
        }
    }

    fn in_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>{
        let mut out_edges:Vec<EdgeIndex> = Vec::new();
        for (from, ref nbs) in self.adj_list.iter().enumerate(){
            for &to in nbs.iter(){
                if to == idx {out_edges.push((from, to))};
            }
        }
        out_edges
    }

    fn out_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>{
        let nb = self.out_nodes(idx);
        nb.iter().map(|&to|{(idx, to)}).collect()
    }

    fn contains_node(&self, node: &V) -> bool{
        match self.node_to_index(node).ok(){
            Some(_) => true,
            None    => false,
        }
    }

    fn contains_edge(&self, from: &V, to: &V) -> bool{
        let fi = match self.node_to_index(from){
            Ok(x)     => x,
            Err(_)    => return false,
        };

        let ti = match self.node_to_index(to){
            Ok(x)     => x,
            Err(_)    => return false,
        };
        self.edges.get(&(fi, ti)).is_some()
    }

    fn out_nodes(&self, idx: NodeIndex)->Vec<NodeIndex>{
        if idx <= self.nodes.len()-1{
            self.adj_list[idx].to_vec()
        } else {
            Vec::new()
        }
    }

    fn nodes(&self) -> Vec<NodeIndex>{
        let res:Vec<NodeIndex> = range(0u, self.nodes.len()).collect();
        res
    }

    fn get_edge(&self, from: NodeIndex, to: NodeIndex)->Option<E>{
        self.edges.find_copy(&(from, to))
    }
}

#[test]
fn graph_with_str_nodes(){
    #[deriving(Clone)]
    struct Edge{
        weight: f64,
        cost: int,
    }
    let edges = vec![
        ("A", "B", Edge{weight: 1.1, cost: 3}),
        ("A", "C", Edge{weight: 1.2, cost: 5}),
        ("B", "C", Edge{weight: 1.3, cost: 5}),
        ("C", "D", Edge{weight: 1.4, cost: 5}),
        ];
    let gp:SparseGraph<&str, Edge> = edges.into_iter().collect();
    assert!(gp.contains_node(&"A"));
    assert!(gp.contains_node(&"B"));
    assert!(gp.contains_node(&"C"));
    assert!(gp.contains_node(&"D"));
    assert!(!gp.contains_node(&"E"));

    assert!(gp.contains_edge(&"A", &"B"));
    assert!(!gp.contains_edge(&"A", &"D"));

    let a_idx = gp.node_to_index(&"A").unwrap();
    let b_idx = gp.node_to_index(&"B").unwrap();
    let c_idx = gp.node_to_index(&"C").unwrap();
    assert_eq!(gp.out_nodes(a_idx), vec![b_idx, c_idx])

    let mut gp_mut = gp;
    assert!(!gp_mut.contains_node(&"E"));
    gp_mut.add_node("E");
    assert!(gp_mut.contains_node(&"E"));
}

#[test]
fn graph_with_int_nodes(){
    let edges = vec![
        (1i, 2i, "Edge 1"),
        (1i, 3i, "Edge 2"),
        (2i, 3i, "Edge 3"),
        (3i, 4i, "Edge 4"),
    ];
    let gp:SparseGraph<int, &str> = edges.into_iter().collect();
    assert!(gp.contains_node(&1i));
    assert!(gp.contains_node(&2i));
    assert!(gp.contains_node(&3i));
    assert!(gp.contains_node(&4));
    assert!(!gp.contains_node(&5));

    assert!(gp.contains_edge(&1i, &2));
    assert!(!gp.contains_edge(&1, &4));
}

