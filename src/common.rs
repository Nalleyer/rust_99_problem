#[derive(Clone, PartialEq, Debug)]
pub enum Node<T> {
    Item(T),
    List(Vec<Node<T>>),
}

pub type List<T> = Vec<Node<T>>;


use quickcheck::{Arbitrary, Gen};
use rand::Rng;
impl<T: Arbitrary> Arbitrary for Node<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Node<T>{
        let is_item: bool = g.gen();
        let size = g.size();
        let s = g.gen_range(0, size);
        arbitrary_node(g, s)
    }
}

fn arbitrary_node<G, T>(g: &mut G, size: usize) -> Node<T> 
    where T: Arbitrary,
          G: quickcheck::Gen,
{
    if size <= 1 {
        Node::Item(Arbitrary::arbitrary(g))
    } else {
        Node::List(arbitrary_vec(g, size / 2))
    }
}

fn arbitrary_vec<G, T>(g: &mut G, size: usize) -> List<T>
    where T: Arbitrary,
          G: quickcheck::Gen,
{
    let sz = g.gen_range(0, size);
    (0..sz).map(|_| arbitrary_node(g, size)).collect()
}
