// error: this function's return type contains a borrowed value,
// but the signature does not say whether it is borrowed from `a` or `b`

// fn max(a: &str, b: &str) -> &str {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

fn max<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        a
    } else {
        b
    }
}


fn main() {
    let mut s = String::from("Hello world");
    let a = &mut s;
    // let b = &mut s;
    println!("{}", a);
    // println!("{}", b);
    println!("{}", max("a", "b"));
}