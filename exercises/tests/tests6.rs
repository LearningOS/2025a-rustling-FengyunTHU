// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
////在这个例子中，我们对Rust标准库的//不安全函数进行了浅显的探究。修正所有的问号和待办事项，使测试//通过。
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

struct Foo {
    a: u128,
    b: Option<String>,
}

/// # Safety
///
/// The `ptr` must contain an owned box of `Foo`.
unsafe fn raw_pointer_to_box(ptr: *mut Foo) -> Box<Foo> {
    // SAFETY: The `ptr` contains an owned box of `Foo` by contract. We
    // simply reconstruct the box from that pointer.// SAFETY:根据约定,“ptr”包含一个“Foo”所属的盒子。我们//简单地从那个指针重建盒子。
    let mut ret: Box<Foo> = unsafe { Box::from_raw(ptr) }; // 从一个智能指针重建
    // todo!("The rest of the code goes here")
    (*ret).b = Some("hello".to_owned()); // 解引用
    ret // 返回值就是Box<Foo>指针
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let data = Box::new(Foo { a: 1, b: None }); // 本身即为引用

        let ptr_1 = &data.a as *const u128 as usize;
        // SAFETY: We pass an owned box of `Foo`.
        let ret = unsafe { raw_pointer_to_box(Box::into_raw(data)) }; // 传入一个Box上Foo的指针

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));
    }
}
