// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
//在本练习中，我们得到一个u32的Vec，称为“numbers”，其值//范围从0到99 - [ 0，1，2，...，98，99 ]我们希望在8个不同的线程中同时使用//这组数字。每个线程//都将得到每八个值的和，并有一个偏移量。////第一个线程(偏移量0)将对0、8、16、...//第二个线程(偏移量1)将1，9，17，...//第三个线程(偏移量2)将求和2，10，18，...// ...//第八个线程(偏移量7)将对7、15、23、...////因为我们使用线程，所以我们的值需要是线程安全的。所以，//我们用的是Arc。我们需要对这两个TODOs分别进行更改。////通过在//第一个TODO注释所在的位置填充“shared_numbers”的值来编译此代码，并在第二个TODO注释所在的位置创建“child _ numbers ”//的初始绑定。尽量不要创建// `numbers` Vec的任何副本！
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.

// I AM DONE

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);// TODO->不需要进行更改，无须Mutex
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);// TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
