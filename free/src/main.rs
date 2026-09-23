use std::thread;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    thread::scope(|s| {
        s.spawn(|| {
            println!("スレッドA: {:?}", numbers);
        });

        s.spawn(|| {
            let sum: i32 = numbers.iter().sum();

            println!("スレッドB: 合計 = {sum}");
        });
    });

    println!("main: {:?}", numbers);
}
