// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
//这是一个试图使用上一个练习中// `total_cost '函数的完整版本的程序。但是没用！//为什么不呢？我们应该做些什么来修复它？
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> { // 为main添加返回值Result
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?; // ?需要对应Result类型的返回值，包含Ok和Err

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(()) // 正常需要返回Ok包围的()
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
