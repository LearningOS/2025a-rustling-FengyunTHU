// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// I AM DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(()) = my_option { // 使用isnone/some，不要使用取反// 或者使用if let
        // my_option.unwrap();->没必要进行unwrap，因为前面已经声明为None
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6 // 添加逗号
    ];
    println!("My array! Here it is: {:?}", my_arr);

    let mut my_empty_vec = vec![1, 2, 3, 4, 5]; // 这样是对vec本身做，返回值是()
    my_empty_vec.clear(); // 使用clear()来替换resize(0,'_')的操作（长度取为0）
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    let mut mider = 0; // 或者命名为MIDER，不要大小写混杂
    // Let's swap these two!
    mider = value_b;
    value_b = value_a;
    value_a = mider;
    println!("value a: {}; value b: {}", value_a, value_b);
}
