fn main() {
    println!("{}", longest("a","bc"));
}
/*

missing lifetime specifier

expected named lifetime parameter

help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}