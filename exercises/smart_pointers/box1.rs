// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//

//在编译时，Rust需要知道一个类型占用了多少空间。这//对于递归类型来说是有问题的，在递归类型中，一个值可以//本身包含另一个相同类型的值。为了解决这个问题，我们可以使用//` box `--一个用于在堆上存储数据的智能指针，它还允许我们//包装一个递归类型。////我们在本练习中实现的递归类型是“cons list ”,这是一种//函数式编程语言中常见的数据结构。cons列表中的每个//项目包含两个元素:当前项目的值和//下一个项目的值。最后一项是一个名为“Nil”的值。////步骤1:在枚举定义中使用“Box”来编译代码//步骤2:通过替换“todo！()`
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 递归使用List
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    // todo!()
    List::Nil
}

pub fn create_non_empty_list() -> List {
    // todo!()
    let mut Nil_box = Box::new(List::Nil);
    for i in 0..5 {
        Nil_box = Box::new(List::Cons(i,Nil_box));
    }
    *Nil_box // 智能指针解引用，在表达式无法自动解引用
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
