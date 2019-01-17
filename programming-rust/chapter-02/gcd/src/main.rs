fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

/// The use declarations birng the two _traits_ Write and FromStr into scope.
/// A trait is a collection of methods that types can implement.
use std::io::Write;
use std::str::FromStr;

fn main() {
    /// Declare a mutable local var numbers, and initialize it to an empty vector.
    /// Vec is like a python list.
    let mut numbers = Vec::new();

    /// std::env::args returns an iterator
    /// The first values produced by the iterator returned by std::env::args
    /// is the name of the program being run so we skip(1) it.
    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}
