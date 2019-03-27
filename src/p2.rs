// P02 (*) Find the last but one box of a list.
// Example:
// * (my-but-last '(a b c d))
// (C D)
#[cfg(test)]
mod tests {
    fn my_last_but_one<T: Clone>(xs: &[T]) -> Vec<T> {
        match xs.len() {
            0...2 => xs.to_vec(),
            _ => xs.iter().skip(xs.len() - 2).cloned().collect()
        }
    }

    #[quickcheck]
    fn last_two_is_first_two(xs: Vec<isize>) -> bool {
        let result = my_last_but_one(&xs);
        match xs.len() {
            0 => result.len() == 0,
            1 => result[0] == xs[0],
            _ => result[0] == xs[xs.len() - 2] && result[1] == xs[xs.len() - 1]
        }
    }
}
