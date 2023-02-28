use adder;

#[test]
fn large_sum() {
    assert_eq!(adder::sum_formula(50, 10_000_000), 50000004998775);
}