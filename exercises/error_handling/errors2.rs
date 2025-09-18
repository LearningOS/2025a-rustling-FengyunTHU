// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//假设我们正在编写一个游戏，你可以用代币购买物品。所有物品的价格为// 5代币，无论何时购买物品，都有1//代币的处理费。游戏的玩家将键入他们想要购买多少物品，并且//‘total _ cost’函数将计算代币的总成本。因为//玩家输入了数量，我们得到的是一个字符串，他们//可能输入了任何东西，而不仅仅是数字！////现在，这个函数根本没有处理错误情况(也没有//正确地处理成功情况)。我们想要做的是:如果我们对不是数字的字符串调用parse '函数，该函数将//返回` ParseIntError ',在这种情况下，我们想要立即从函数中返回//该错误，而不是尝试乘法和加法。////至少有两种实现方法都是正确的，但是其中一种//要短得多！
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?; // 使用问号错误会直接返回，正确的话则继续
    Ok(qty * cost_per_item + processing_fee)
    // match item_quantity.parse::<i32>() {
    //     Ok(qty) => Ok(qty * cost_per_item + processing_fee),
    //     Err(e) => return Err(e) // 直接返回一个Err(e)即可，无须指定，ParseIntError即为识别的类型
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
