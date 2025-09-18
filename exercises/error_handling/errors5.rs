// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
//这个程序使用了errors4中代码的修改版本。////这个练习使用了一些我们在//课程后面才会用到的概念，比如“Box”和“From”特征。现在详细理解它们并不重要，但是如果你愿意，你可以继续往下读。现在，想想“盒子< dyn？？？>`键入“我想要任何有用的东西？？?"类型，//考虑到Rust通常的运行时安全标准，您应该会觉得//有些宽松！////简而言之，盒子的这个特殊用例是当你想拥有一个//值，并且你只关心它是一个实现特殊//特征的类型时使用的。为此，将Box声明为Box<dyn Trait >类型，其中Trait是//编译器在该上下文中使用的任何值上查找的Trait。对于这个//练习，上下文是可能在//结果中返回的潜在错误。////我们可以用什么来描述这两个错误？换句话说，有没有一个特质//两个错误都实现了？
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> { // 一个包括所有的error的错误，抽象层次最高
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
