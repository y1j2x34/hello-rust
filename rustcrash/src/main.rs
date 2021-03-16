// let z = 0; // 变量不能声明在全局
const Z: i32 = 0; // 常量可以全局声明，并且要求声明类型以及变量名要大写
fn main() {
    let x = 1;
    // x = 42; // x 是不可变的
    let mut y = 1; // y是可变的
    println!("y = {}", y);
    y = 42;
    println!("{} {} {}", x, y, Z);

    let string_no_type = "Hello World";
    let string_with_type: &str = "Hello world";
    println!("{}", string_no_type);
    println!("{}", string_with_type);


    let dbl = 3.0; // f64
    let flt:f32 = 3.0; // f32

    println!("f64: {}, f32: {}", dbl, flt);

    // char Unitcode 码, 4Byte, 支持emoji表情等
    
    let char_ascii = 'a';
    let char_emoji = '☞';

    println!("ascii {}, emoji {}", char_ascii, char_emoji);

    array_demo();
    process_control();
    luup();
}

fn array_demo() {
    let array: [i32; 5] = [1,2,3,4,5];
    println!("{}", array[0]);

    for n in array.iter() {
        println!("array: {}", n);
    }
}

fn process_control() {
    fn a(num: i32) {
        if num < 5 {
            println!("less than 5")
        } else {
            println!("great or equals to 5")
        }
    }
    a(5);
    a(1);
    a(6);
}

fn luup() {
    let mut number = 10;
    while number > 0 {
        println!("number: {}", number);
        number -= 1;
    }

    'outer: loop {
        loop {
            println!("'loop, number {}", number);
            number += 1;
            if number > 10 {
                break 'outer;
            }
        }
    }
    for i in 0..5 {
        println!("for loop: {}", i);
    }
}