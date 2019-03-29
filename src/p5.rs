// P05 (*) Reverse a list.
#[cfg(test)]
mod tests {
    fn my_rev<T: Clone>(xs: &[T]) -> Vec<T>{
        xs.iter().rev().cloned().collect()
    }

    #[quickcheck]
    fn first_is_last(xs: Vec<isize>) -> bool {
        let result = my_rev(&xs);
        let len = xs.len();
        for (i, value) in result.iter().enumerate() {
            if *value != xs[len - i - 1] {
                return false
            }
        }
        true
    }
}
