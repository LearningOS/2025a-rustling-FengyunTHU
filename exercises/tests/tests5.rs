// tests5.rs
//
// An `unsafe` in Rust serves as a contract.
//
// When `unsafe` is marked on an item declaration, such as a function,
// a trait or so on, it declares a contract alongside it. However,
// the content of the contract cannot be expressed only by a single keyword.
// Hence, its your responsibility to manually state it in the `# Safety`
// section of your documentation comment on the item.
//
// When `unsafe` is marked on a code block enclosed by curly braces,
// it declares an observance of some contract, such as the validity of some
// pointer parameter, the ownership of some memory address. However, like
// the text above, you still need to state how the contract is observed in
// the comment on the code block.
//
// NOTE: All the comments are for the readability and the maintainability of
// your code, while the Rust compiler hands its trust of soundness of your
// code to yourself! If you cannot prove the memory safety and soundness of
// your own code, take a step back and use safe code instead!
//
//Rust中的“unsafe”充当合同。////当“unsafe”被标记在一个项声明上时，比如一个函数、//一个特征等等，它在它旁边声明一个约定。但是//契约的内容不能只用单个关键字来表达。//因此，您有责任在该项的文档注释的`# Safety`//部分手动声明它。////当用花括号括起来的代码块上标记了“unsafe”时，//它声明了对某个约定的遵守，比如某个//指针参数的有效性，某个内存地址的所有权。然而，就像//上面的文本一样，您仍然需要在//代码块的注释中说明如何遵守契约。////注意:所有的注释都是为了//你的代码的可读性和可维护性，而Rust编译器把对//你的代码的可靠性的信任交给了你自己！如果您不能证明//您自己的代码的内存安全性和可靠性，请后退一步，改用安全代码！

// Execute `rustlings hint tests5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {
    // TODO: Fill your safety notice of the code block below to match your
    // code's behavior and the contract of this function. You may use the
    // comment of the test below as your format reference.
    // TODO:填写下面代码块的安全声明，以匹配您的//代码行为和此函数的约定。您可以使用下面测试的//注释作为您的格式参考。
    unsafe {
        // todo!("Your code goes here")
        *(address as *mut u32) = 0xAABBCCDD; // 做强制类型转换，usize声明为*mut u32，地址是十六进制可以转化为int
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) }; // 将&mut t转换为*mut u32，然后将(*mut u32)转换为usize
        assert!(t == 0xAABBCCDD);
    }
}
