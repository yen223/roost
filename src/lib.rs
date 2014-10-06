mod sparse_graph;
mod search;

mod roost{
    pub use sparse_graph::SparseGraph;
    // pub use search::depth_first_visit;

    pub type NodeIndex = uint;
    pub type EdgeIndex = (NodeIndex, NodeIndex);
    pub trait Graph<V, E>{
        fn index_of(&self, node: &V) -> Option<NodeIndex>;
        fn node_of(&self, idx: NodeIndex) -> Option<V>;
        fn neighbors(&self, idx: NodeIndex) -> Vec<NodeIndex>;
        fn in_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn out_edges(&self, idx: NodeIndex) -> Vec<EdgeIndex>;
        fn contains_node(&self, node: &V) -> bool;
        fn contains_edge(&self, from: &V, to: &V) -> bool;
        fn nodes(&self) -> Vec<NodeIndex>;
        fn get_edge(&self, from: NodeIndex, to: NodeIndex) -> Option<E>;
    }
}
