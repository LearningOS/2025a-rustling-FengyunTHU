// tests3.rs
//
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the
// result we expect to get when we call `is_even(5)`.
//
//这个测试不是测试我们的函数，让它以测试通过的方式来测试。然后编写第二个测试，测试我们调用` is_even(5)`时是否得到了//我们期望得到的结果。
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(10));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}
