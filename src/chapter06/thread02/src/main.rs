use std::thread;
use std::time::Duration;

fn spawn_function() {
    for i in 0..5 {
        println!("子线程 {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    for i in 0..3 {
        println!("主线程 {}", i);
        let handle =  thread::spawn(move||spawn_function());
        handle.join().unwrap();
        thread::sleep(Duration::from_millis(1));
    }
}