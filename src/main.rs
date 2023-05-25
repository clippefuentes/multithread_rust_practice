use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let thread1 = thread::spawn(move|| {
        for val in v1 {
            thread::sleep(Duration::from_secs(1));
            tx.send(val).unwrap();
        }
    });

    let thread2 = thread::spawn(move|| {
        for recvd in rx {
            println!("Data Received = {}", recvd);
        }
    });

    thread2.join().unwrap();
}
