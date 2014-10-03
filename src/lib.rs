mod sparse_graph;
mod search;

mod roost{
    pub use sparse_graph::SparseGraph;
    pub use search::depth_first_visit;
    use std::hash::Hash;
    use std::hash::sip::SipState;

    pub trait Graph<N>{
        fn contains_node(&self, node: N) -> bool;
        fn contains_edge(&self, from: N, to: N) -> bool;
        fn neighbors(&self, node: N) -> Vec<N>;
    }

    #[deriving(PartialEq, Clone)]
    pub struct Edge<N>{
        pub from: N,
        pub to: N,
        pub weight: f64,
    }
    
    // pub type Neighbors<N> = 
    //     idx: int,
    //     nodes: Vec<N>,
    // }

    impl<N:Hash> Hash for Edge<N> {
        fn hash(&self, state: &mut SipState) {
            self.from.hash(state);
            self.to.hash(state);
        }
    }

    // impl<N> Iterator<N> for Neighbors<N> {
    //     fn next(&mut self) -> Option<N> {
            
}
