my Str $content = q:to/END/;
// problem here
#[cfg(test)]
mod tests {
    fn function<T: Clone>(xs: &[T]) -> Vec<T> {
    }

    #[quickcheck]
    fn check_function(xs: Vec<isize>) -> bool {
    }
}
END
sub MAIN(Int $n) {
    "./src/p$n.rs".IO.spurt($content);
}
