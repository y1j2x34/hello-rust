fn main() {
    // Rust 里面，每个值都必须要有“一个”所有者
    // 每个值都有一个作用域，当这个值离开作用域后，就会被销毁。
    let string1 = String::from("hello world");
    println!("string1 {}", string1);
    {
        let string2 = String::from("string2");
        println!("string2 {}", string2);
        let string4 = string1; // 修改了值的所有者， string1 的所有权转移到 string3 了。
        println!("string4 {}", string4);
    }
    // ! compile error
    // println!("string2 {}", string2); // Cannot find value 'string2' in this scope 
    // let string3 = string1; // string1 的所有权已经被转移了
    // println!("string3 {}", string1); // 这行不能通过编译
}
/*
 * 常见内存管理方式
 * 
 * 1. C 语言的 malloc 和 free (手动管理)
 * 2. GC： Golang, Java 等（自动管理）
 * 3. 基于生命周期的半自动管理： Rust
 * 
 */

 /*
  理解生命周期
  1. C 语言中需要手动调用 free 释放内存
  2. Rust 在编译期间计算变量的使用范围
  3. 当变量不再被使用时，编译器自动在源码中插入free代码。
 */