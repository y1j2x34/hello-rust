fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

fn reverse_string2(s: &String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let s1 = String::from("Hello world");

    let _s2 = reverse_string(s1); // s1 的所有权转移给 函数， 所以后面的 s1 就不能用了

    let s3 = String::from("Hello world");

    let s4 = reverse_string2(&s3); // 使用引用类型

    // println!("str: {}, after_reverse {}", s1, _s2);
    println!("str: {}, after_reverse {}", s3, s4);
}
/*
引用的规则

1. 不会获取所有权
2. 默认情况是不可变的
3. 同一时间最多只能存在一个可变引用

*/