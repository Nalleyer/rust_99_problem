//(**) Flatten a nested list structure.
// Transform a list, possibly holding lists as elements into a `flat' list by replacing each list with its elements (recursively).
// 
// Example:
// * (my-flatten '(a (b (c d) e)))
// (A B C D E)
// 
// Hint: Use the predefined functions list and append.

use crate::common::*;
// pub fn flat<T: Clone>(ls: List<T>) -> List<Node<T>> {
//     ls.map(|&x| x)
// }

#[cfg(test)]
mod tests {
    use crate::common::*;
    // #[quickcheck]
    // fn check_function(ls: List<isize>) -> bool {
    //     true
    // }
}
