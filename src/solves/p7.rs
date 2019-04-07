//(**) Flatten a nested list structure.
// Transform a list, possibly holding lists as elements into a `flat' list by replacing each list with its elements (recursively).
// 
// Example:
// * (my-flatten '(a (b (c d) e)))
// (A B C D E)
// 
// Hint: Use the predefined functions list and append.

use crate::common::*;
pub fn flat<T: Clone>(ls: &List<T>) -> List<T> {
    ls.iter().flat_map(|node|
        match node {
            Node::Item(x) => vec!(node.clone()),
            Node::List(ls) => ls.clone()
        }
    ).collect()
}

#[cfg(test)]
mod tests {
    use crate::common::*;
    /*
    #[quickcheck]
    fn check_function(ls: List<i32>) -> bool {
        // println!("{:?}", ls);
        if ls.len() > 0 {
            match &ls[0] {
                Node::Item(x) => true,
                Node::List(v) => false
            }
        } else {
            true
        }
    }
    */
    
    #[test]
    fn empty() {
        let nested: List<isize> = vec![Node::Item(3), Node::List(vec!(Node::Item(5), Node::Item(7)))];
        let flatted = super::flat(&nested);
        assert_eq!(flatted, vec![Node::Item(3), Node::Item(5), Node::Item(7)]);
    }
}
