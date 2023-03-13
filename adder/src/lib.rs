#![feature(test)]
extern crate test;

use rayon::prelude::*;

pub fn sum_primitive(start: u64, end: u64) -> u64 {
    let vector: Vec<u64> = (start..=end).collect();

    vector.iter().sum()
}

pub fn sum_primitive_par(start: u64, end: u64) -> u64 {
    let vector: Vec<u64> = (start..=end).collect();

    vector.par_iter().sum()
}

pub fn sum_formula(start: u64, end: u64) -> u64 {
    let start: u64 = if start == 0 { 1 } else { start };
    (end - start + 1) * (start + end) / 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn primitive(b: &mut Bencher) {
        b.iter(|| sum_primitive(0, 5_000_000));
    }

    #[bench]
    fn primitive_par(b: &mut Bencher) {
        b.iter(|| sum_primitive_par(0, 5_000_000));
    }

    #[bench]
    fn formula(b: &mut Bencher) {
        b.iter(|| sum_formula(0, 5_000_000));
    }

    #[test]
    fn it_works() {
        assert_eq!(sum_formula(0, 5_000_000), sum_primitive_par(0, 5_000_000));
        assert_eq!(sum_formula(1, 100), sum_primitive(1, 100));
        assert_eq!(sum_formula(50, 2_000_000), sum_primitive(50, 2_000_000));
    }
}
