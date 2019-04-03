// P01 (*) Find the last box of a list.
// Example:
// * (my-last '(a b c d))
// (D)
#[cfg(test)]
mod tests {
    fn my_last<T: Clone>(xs: &[T]) -> Vec<T> {
        match xs.last() {
            Some(x) => vec![x.clone()],
            None => vec![]
        }
    }

    #[quickcheck]
    fn last_is_first_of_reversed(xs: Vec<isize>) -> bool {
        let res = my_last(&xs);
        match res.len() {
            0 => xs.len() == 0,
            _ => res[0] == xs.iter().rev().next().unwrap().clone()
        }
    }
}
