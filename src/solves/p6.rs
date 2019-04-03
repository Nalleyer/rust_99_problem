// P06 (*) Find out whether a list is a palindrome.
// A palindrome can be read forward or backward; e.g. (x a m a x).

extern crate rand;

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    
    fn is_palindrome<T: PartialEq>(xs: &[T]) -> bool {
        xs.iter().zip(xs.iter().rev()).all(|(x, y)| (*x) == (*y))
    }

    #[test]
    fn empty() {
        assert_eq!(is_palindrome::<isize>(&[]), true);
    }
    
    #[test]
    fn single() {
        assert_eq!(is_palindrome(&[false]), true);
    }
    
    fn random_list(len: usize) -> Vec<usize> {
        let mut rng = thread_rng();
        (0..len).map(|_| rng.gen::<usize>()).collect()
    }
    
    #[test]
    fn random() {
        const LEN: usize = 20;
        for _ in 1..20 {
            let mut list = random_list(LEN);
            let r_list: Vec<usize> = list.iter().rev().cloned().collect();
            list.extend(&r_list);
            assert_eq!(is_palindrome(&list), true);
        }
    }
}
