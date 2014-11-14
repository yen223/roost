#![crate_name = "roost"]
#![crate_type="lib"]

//! A graph library, written in pure Rust.

mod adjacency_list;
mod graph_error;
mod path;
mod traversal;
mod edge;

pub mod graph{
    pub use adjacency_list::AdjList as AdjList;
    pub use graph_error as error;
    pub use edge;
    pub use path;
    pub use traversal;
    
    pub type NodeIndex = uint;
    pub type EdgeIndex = (NodeIndex, NodeIndex);
    pub type Node = Result<NodeIndex, error::GraphError>; 
    pub type Edge = Option<EdgeIndex>;

    /// The base Graph trait. Provides basic operations to access, add, and 
    /// remove nodes and edges in a graph. Graphs can take any cloneable type 
    /// for Nodes and Edges. 
    pub trait Graph<V, E>{
        fn insert_node(&mut self, node: V) -> NodeIndex;
        fn node_to_index(&self, node: &V) -> Node;
        fn index_to_node(&self, idx: NodeIndex) -> Option<V>;
        fn out_nodes(&self, idx: NodeIndex) -> Vec<NodeIndex>;
        fn in_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn out_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn contains_node(&self, node: &V) -> bool;
        fn contains_edge(&self, from: &V, to: &V) -> bool;
        fn nodes(&self) -> Vec<NodeIndex>;
        fn get_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<E>;
    }

    /// Locates the NodeIndex of a generic type for a given graph.
    /// Returns None if not found.
    /// All public functions and methods which take a node as an input
    /// should use Node as the input type.
    pub fn node<V,E> (item: &V, graph:&Graph<V,E>) -> Node {
        graph.node_to_index(item)
    }
}
