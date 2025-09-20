// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//这个程序产生多个线程，每个线程至少运行250毫秒，每个线程返回它们完成所用的时间。该程序应该等到所有产生的线程都已经完成，并且应该将它们的返回值收集到一个向量中。
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i); // 拿走i的所有权
            start.elapsed().as_millis()
        })); // 创建一个handle的Vec
    }

    let mut results: Vec<u128> = vec![];
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?\
        results.push(handle.join().unwrap()); // handle.join()可以提供返回值，为Result类型
        // match handle.join() {
        //     Ok(x) => results.push(x),
        //     Err(e) => Err(e), // 【这里两个需要是相同的返回类型，上面Ok为()，下面为result，不符合】
        // };
        // results.push(handle);
    }

    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
