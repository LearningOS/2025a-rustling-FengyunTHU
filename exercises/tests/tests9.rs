// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//

// Rust具有很强的与C/C++和其他静态编译//语言共享FFI接口的能力，甚至可以在代码本身内部进行链接！它通过了extern//块，就像下面的代码一样。////关键字“extern”后的短字符串指示外部导入的//函数将跟随哪个ABI。在本练习中，使用了“Rust ”,而存在其他变体，如//“C”代表标准C ABI，“stdcall”代表Windows ABI。////外部导入的函数在extern块中声明，用分号//而不是花括号来标记签名的结尾。一些属性可以应用于那些//函数声明来修改链接行为，比如#[link_name = " .. "]来//修改实际的符号名称。////如果要将符号导出到链接环境中，也可以//在函数定义之前用相同的ABI字符串注释标记“extern”关键字。Rust函数的默认ABI//就是字面上的“Rust”，所以如果你想链接纯Rust函数，//整个extern项都可以省略。//// Rust默认情况下会破坏符号，就像C++一样。为了抑制这种行为并使//这些函数可通过名称寻址，可以应用属性#[no_mangle]。////在本练习中，您的任务是让测试用例能够调用//模块Foo中的“my_demo_function”。“my_demo_function_alias”是“my_demo_function”的别名，因此测试用例中的两行//代码应该调用同一个函数。////除了添加两行属性之外，不应修改任何现有代码。

// You should NOT modify any existing code except for adding two lines of attributes.

// I AM DONE

extern "Rust" {
    #[link_name = "Alpfat"] // 设置链接到的导出的名称，在extern中声明接口时使用
    fn my_demo_function(a: u32) -> u32;
    #[link_name = "Alpfat"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[export_name = "Alpfat"] // 设置导出的名称，在函数实现的时候，无须extern关键字
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        ////默认情况下，外部导入的函数是不安全的//因为其他语言的来源不受信任。您可以//将它们包装在安全的Rust APIs中，以减轻调用者的负担。// //安全:我们知道这些函数是安全//信任函数的别名。
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
