fn main() {
    println!("{}", max(10, 12));
    println!("{}", generic_max::<u32>(10, 12)); // 类型可以省略，由编译器自动推断
    println!("{}", generic_max(10, 12));

    option_demo();
}

fn generic_max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn max(a: u32, b: u32) -> u32 {
    if a > b {
        a
    } else {
        b
    }
}
fn option_demo() {
    let homedir = std::env::home_dir();
    // Option => 有 和 无
    match homedir {
        Some(data) => println!("option is some, data = {:?}", data),
        None => println!("option is none")
    }
    // Result => 对和错
    match std::env::var("LANG") {
        Ok(data) => println!("ok! {:?}", data),
        Err(err) => println!("err {:?}", err)
    }
    match std::env::var("NotExist") {
        Ok(data) => println!("ok! {:?}", data),
        Err(err) => println!("err {:?}", err)
    }
}