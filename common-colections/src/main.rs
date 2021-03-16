fn main() {
    string_demo();
    hash_map();
}

fn hash_map() {
    use std::collections::HashMap;
    let mut map: HashMap<String, String> = HashMap::new();
    let key = String::from("Favorite color");
    let value = String::from("Blue");

    map.insert(key, value);
    // key, value 所有权已经被 map 剥夺
    // println!("{} {}", key, value); // borrow of moved value
    for (key, value) in map.iter() {
        println!("{} => {}", key, value);
    }

}

#[allow(dead_code)]
fn string_demo() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // ! &String 会被强转成 &str(解引用强制多态技术)
    let s4 = s3 + "";
    println!("{}", s4);

    // 使用 format! 拼接
    let s1 = String::from("tik");
    let s2 = String::from("tok");
    let s3 = format!("{}{}", s1, s2); // ! 不会占用所有权
    println!("{}", s3);
    println!("{}", s1);

    let s1 = String::from("中文");
    assert_eq!(s1.len(), 6);
    let first_chr = &s1[0..3];
    println!("first character is '{}'", first_chr);
    let s2 = String::from("English");
    
    let first_char = s1.chars().next().unwrap();
    println!("first character is {}", first_char);
    
    let first_char = &s2[0..1]; // 只能取到 单字节字符
    println!("first character is {}", first_char);
    
    for c in (&s1).chars() {
        print!("{}", c);
    }

    let first = &s1[0..1]; // ! panicked, 字节索引1不是字符边界，'中'实际上会占用了3个字节（UTF8）
    println!("{}", first);

}
#[allow(dead_code)]
fn vector_demo() {
    let mut vv:Vec<i32> = Vec::new();
    vv.push(2);

    let v = vec!(1,2,3,4,5);
    for i in &v {
        println!("{}", i);
    }
    println!("{}", v[0]);
    // 同时拥有多个不同类型数据的 vector 
    enum SpreadsheetCell {
        Int(i32),
        Text(String),
        Float(f32)
    }

    let row = vec![
        SpreadsheetCell::Int(12),
        SpreadsheetCell::Float(12.0),
        SpreadsheetCell::Text(String::from("asd")),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(v) => println!("int({})", v),
            SpreadsheetCell::Float(v) => println!("float({})", v),
            SpreadsheetCell::Text(v) => println!("text({})", v),
        }
    }
}
/*
fn borrow() {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(5); // cannot borrow `v` as mutable because it is also borrowed as immutable
    println!("{}", first);
}
*/