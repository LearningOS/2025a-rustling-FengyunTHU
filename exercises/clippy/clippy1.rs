// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Clippy工具是一个lints集合，用于分析您的代码，因此您可以//捕捉常见错误并改进您的Rust代码。////对于这些练习，当出现clippy//警告时，代码将无法编译。请检查输出中clippy的建议以解决该练习。
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::f32;

fn main() {
    let pi = std::f32::consts::PI; // 使用正确合理的pi值而不是自定义
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
