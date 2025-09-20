// lifetimes2.rs
//
// So if the compiler is just validating the references passed to the annotated
// parameters and the return type, what do we need to change?
////那么如果编译器只是验证传递给带注释的//参数的引用和返回类型，我们需要改变什么呢？
// Execute `rustlings hint lifetimes2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    let string2 = String::from("xyz");
    {
        // let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is '{}'", result);
}
