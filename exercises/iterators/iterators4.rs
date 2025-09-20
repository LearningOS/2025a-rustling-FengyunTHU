// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

pub fn factorial(num: u64) -> u64 {
    ////完成此函数返回num的阶乘//不要使用:// - return //尽量不要使用:// -命令式循环(for，while) // -附加变量//对于额外的挑战，不要使用://-recursion//Execute ` rustlings hint iterators 4 `进行提示。
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    let mut result:u64 = 1;
    let mut nums:u64 = num;
    loop {
        match nums > 0 {
            true => {
                result*=nums;
                nums = nums -1;
            },
            false => break,
        };
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
