fn main() {
    let s1 = String::from("Hello");
    println!("length of s1: {}", calc_len1(s1));

    let s1 = String::from("Hello");
    let (s2, len) = calc_len2(s1); // s1 所有权转给 calc_len2 后再被移交出去。
    println!("s2: {}, len: {}", s2, len);

    // println!("{}", s1) // 不能通过编译， s1所有权移到 calc_len1 函数里，所以不再有效
    let s1 = String::from("hello");
    println!("length of s1: {}", calc_len3(&s1));
    println!("{}", s1); // 上面使用了引用，它允许 calc_len3 使用值但是没有所有权

    let mut s1 = String::from("Hello");
    let len = calc_len4(&mut s1);// s1的值将被改变
    println!("s1:{}, len: {}", s1, len);

    // 特定作用域中的特定数据只能有一个可变引用
    /*
    let mut s = String::from("Hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    // let r3 = &mut s; // 不能同时拥有不可变引用和可变引用。


    let mut s = String::from("Hello world");

    let wordLastIndex = first_word(&s);
    let hello = &s[..wordLastIndex];
    println!("first word is {}", hello);

    s.clear();

    assert_eq!(s, "");
}

fn calc_len1(s: String) -> usize {
    s.len()
}
fn calc_len2(s: String) -> (usize, String) {
    (s.len(), s)
}
fn calc_len3(s: &String) -> usize {
    // s的作用域与函数的作用域一样，不过由于没有所有权，所以离开作用后不会丢弃指向的数据。
    s.len()
}

fn calc_len4(s: &mut String) -> usize {
    s.push_str(" World");
    s.len()
} 

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
