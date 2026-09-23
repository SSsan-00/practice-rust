use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("スレッドA: {i}");

            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("スレッドB: {i}");

            thread::sleep(Duration::from_millis(500));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("全てのスレッドが終了しました!");
}
