// P03 (*) Find the K'th element of a list.
// The first element in the list is number 1.
// Example:
// * (element-at '(a b c d e) 3)
// C
#[cfg(test)]
mod tests {
    fn my_nth<T: Clone>(xs: &[T], n: usize) -> Option<T> {
        match n {
            0 => None,
            _ => xs.iter().nth(n - 1).cloned()
        }
    }

    #[quickcheck]
    fn nth_is_first_after_skip_n_minus_1(xs: Vec<isize>, n: usize) -> bool {
        let result = my_nth(&xs, n);
        match n {
            0 => result == None,
            _ => xs.iter().skip(n - 1).nth(0).cloned() == result
        }
    }
}
