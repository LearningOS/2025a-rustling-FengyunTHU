// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
// Rust中的类型转换是通过使用“as”运算符完成的。请注意//“as”运算符不仅在类型转换时使用。它还有助于//重命名导入。//
//目标是确保除法不会编译失败，并且//返回正确的类型。
//
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / (values.len() as f64) // values.len()是usize，做除法要类型相同，total是f64，需要len也转换为f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
