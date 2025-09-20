// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//在上一个练习的基础上，我们希望所有的线程都能够完成它们的工作，但是这一次衍生的线程需要负责更新共享值:JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;
// 引入Mutex
use std::sync::Mutex;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // Arc<JobStatus>
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // 在多线程中都更改同一个数据的时候需要使用锁，确保只有一个进程在进行更改
            let mut status_shared_inner = status_shared.lock().unwrap(); // 锁
            status_shared_inner.jobs_completed+=1; // 使用指针访问->实现智能指针可以直接进行内部修改
            // Arc支持在多线程之间安全使用数据，但是都是不可变引用->Mutex可以用于更改
            // Mutex内部的数据可以更改
            // status_shared.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        println!("jobs completed {}", status.lock().unwrap().jobs_completed);
    }
}
