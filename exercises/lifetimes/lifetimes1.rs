// lifetimes1.rs
//
// The Rust compiler needs to know how to check whether supplied references are
// valid, so that it can let the programmer know if a reference is at risk of
// going out of scope before it is used. Remember, references are borrows and do
// not own their own data. What if their owner goes out of scope?
//
//Rust编译器需要知道如何检查所提供的引用是否//有效，以便它可以让程序员知道引用在使用前是否//有超出范围的风险。记住，引用是借用的，并不拥有自己的数据。如果它们的所有者超出范围怎么办？
// Execute `rustlings hint lifetimes1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // 标注生命周期->x,y,返回值的生命周期相同，只要x和y有效则返回值有效
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
