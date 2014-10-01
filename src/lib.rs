mod sparse_graph;

mod roost{
    use std::hash::Hash;
    use std::hash::sip::SipState;
    pub trait Graph<N>{
        fn contains_node(&self, node: N) -> bool;
        fn contains_edge(&self, from: N, to: N) -> bool;
        // fn neighbors(&self, node: N) -> Iterator<N>;
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
