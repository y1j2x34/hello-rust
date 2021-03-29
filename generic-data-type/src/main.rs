fn main() {
    println!("Hello, world!");
}

#[allow(dead_code)]
struct GenericStruct<T> {
    x: T
}
// 定义具体类型
impl GenericStruct<i32> {

}
// T 声明到 impl 后面，表示泛型而不是具体类型
impl<T> GenericStruct<T> {
    // 方法和结构体定义不同的泛型类型
    fn mixup<V>(&self, other: GenericStruct<V>) -> GenericStruct<V> {
        GenericStruct {
            x: other.x
        }
    }
}

#[allow(dead_code)]
fn generic_fn<T>(v: T) -> T {
    v
}