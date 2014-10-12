#![crate_name = "roost"]
#![crate_type="lib"]

//! A graph library, written in pure Rust.

mod sparse_graph;
mod search;
mod edge;

mod roost{
    pub use sparse_graph::SparseGraph;
    pub use edge;
    pub use search;
    // pub use search::depth_first_visit;

    pub type NodeIndex = uint;
    pub type EdgeIndex = (NodeIndex, NodeIndex);
    
    /// The base Graph trait. Provides basic operations to access, add, and 
    /// remove nodes and edges in a graph. Graphs can take any cloneable type 
    /// for Nodes and Edges. 
    pub trait Graph<V, E>{
        fn insert_node(&mut self, node: V) -> NodeIndex;
        fn index_of(&self, node: &V) -> Option<NodeIndex>;
        fn node_of(&self, idx: NodeIndex) -> Option<V>;
        fn out_nodes(&self, idx: NodeIndex) -> Vec<NodeIndex>;
        fn in_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn out_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn contains_node(&self, node: &V) -> bool;
        fn contains_edge(&self, from: &V, to: &V) -> bool;
        fn nodes(&self) -> Vec<NodeIndex>;
        fn get_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<E>;
    }

    fn node<V,E> (item: &V, graph:&Graph<V,E>)->Option<NodeIndex>{
        graph.index_of(item)
    }
}
