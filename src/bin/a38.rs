// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread::spawn;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let hello: std::thread::JoinHandle<&'static str> = spawn(|| {
        let msg = msg_hello();
        msg
    });

    let threads: std::thread::JoinHandle<&'static str> = spawn(|| {
        let msg = msg_thread();
        msg
    });

    let exited: std::thread::JoinHandle<&'static str> = spawn(|| {
        let msg = msg_excited();
        msg
    });

    let hello = hello.join().unwrap().to_string();
    let threads = threads.join().unwrap().to_string();
    let exited = exited.join().unwrap().to_string();
    println!("{}{}{}", hello, threads, exited)
}
