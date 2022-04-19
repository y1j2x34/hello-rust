// 注意：
// 字符串切片的范围索引必须发生在有效的UTF-8字符边界内
// 如果尝试从一个多字节的字符串中创建字符串切片，程序会报错并退出

// 字符串字面值是切片
// 字符串字面值是被直接存储在二进制程序中
// let s = "Hello, World"
// 变量 s 的类型是 &str, 它是一个指向二进制程序特定位置的切片
//      &str 是不可变引用， 所以字符串字面值也是不可变的


// 将字符串切片作为参数传递
// 用 &str 作为参数类型，，这样就可以同时接收 String 和 &str 类型的参数了。


fn main() {
    println!("Hello, world!");

    let mut s = String::from("Hello world");


    // let h = &s[0..5]; // 字符串切片： [开始索引（起始位置索引值）..结束索引（终止位置的下一个索引值）]
    // let w = &s[6..];
    // let whole = &s[0..];
    // println!("{} {}, {}", h, w, whole);

    let wordIndex = first_word(&s[..]);
    // s.clear(); // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    //
    println!("{}", wordIndex);

}

fn first_word_len(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '  {
            return &s[..i];
        }
    }
    &s[..]
}