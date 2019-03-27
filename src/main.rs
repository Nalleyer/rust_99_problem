#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod p2;

fn main() {
    println!("99 problem solved by rust, change 'mod p1' to any problem you want to test, for example 'mod p99'");
}
