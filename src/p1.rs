// P01 (*) Find the last box of a list.
// Example:
// * (my-last '(a b c d))
// (D)
#[cfg(test)]
mod tests {
    fn my_last<T>(xs: &[T]) -> Option<&T> {
        xs.last()
    }

    #[quickcheck]
    fn last_is_first_of_reversed(xs: Vec<isize>) -> bool {
        my_last(&xs) == xs.iter().rev().next()
    }
}
