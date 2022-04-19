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
什么是所有权

内存通过一个所有权系统来管理，其中包含一组编译器在编译时检查的规则。程序运行时，所有权特性不会减慢程序的运行速度。

所有权规则

1. 每个值都有一个变量，这个变量是该值的所有者
2. 每个值同时只能有一个所有者
3. 当所有者超出作用域时，该值将被删除

*/

/*

Stack vs Heap

Stack:

按照接受顺序存储，按相反顺序移除（后进先出，LIFO）
    - 添加数据叫压入栈（压栈）
    - 移除数据叫做弹出栈（出栈）

所有存储在 Stack 上的数据必须拥有已知的固定大小
编译时大小为止的数据或者运行是大小可能发生变化的数据必须存放在 heap 上。

Heap 内存组织性差一些：
    - 当你把数据放入 heap 是，你会请求一定数量的空间
    - 操作系统在heap里找到一块足够大的空间，把它标记为在用，并返回一个指针，也就是这个空间的地址。
    - 这个过程叫做在 heap 上进行“分配“
*/
