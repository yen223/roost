pub trait DistanceEdge<N>:Clone 
    where N: Num+ToPrimitive,
{
    fn distance(&self) -> N;
}

