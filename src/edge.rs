pub trait DistanceEdge<N>:Clone 
    where N: Num+ToPrimitive,
{
    fn distance(&self) -> N;
}


#[deriving(Eq, PartialEq, Clone)]
pub struct UnitEdge;

impl DistanceEdge<int> for UnitEdge
{
    fn distance(&self) -> int {1}
}
