use std::thread;
use std::time::Duration;

fn main() {
    println!("{}", simulated_expensive_calculation(5))
}

fn simulated_expensive_calculation(idensity: u32) -> u32 {
    thread::sleep(Duration::from_secs(2));
    idensity
}

type Func = Fn(u32) -> u32;

fn memorize(callback: fn(u32) -> u32) -> impl Fn(u32) -> u32 {
    let cacheValue: Option<u32> = None;

    move |value: u32| {
        if cacheValue == None {
            cacheValue = Some(callback(value));
        }
        cacheValue.take()
    }
}
