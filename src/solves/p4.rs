// P04 (*) Find the number of elements of a list.
#[cfg(test)]
mod tests {
    fn my_len<T: Clone>(xs: &[T]) -> usize {
        xs.len()
    }

    #[quickcheck]
    fn len_is_len(xs: Vec<isize>) -> bool {
        xs.len() == my_len(&xs)
    }
    
    #[quickcheck]
    fn skip_len_minus_1_is_last(xs: Vec<isize>) -> bool {
        let len = my_len(&xs);
        match len {
            0 => xs.len() == 0,
            _ => xs.iter().skip(len - 1).nth(0).cloned() == xs.iter().last().cloned()
        }
    }
}
