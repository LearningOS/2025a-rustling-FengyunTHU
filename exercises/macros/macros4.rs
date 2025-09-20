// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => { // 修改宏匹配模式->$()匹配括号内部的模式，而模式是$val:expr，匹配任何表达式并赋值为val
        // 这里只匹配一个，所以包含即可；如果多个字符需要$($val:expr),*，指定分隔符，取匹配多个这样的模式
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
