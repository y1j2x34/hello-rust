struct User {
    email: String,
    name: String,
    sign_in_count: u64
}

impl User {
    pub fn to_string(self) -> String {
        format!("name = {}, email = {}, sign in count = {}", self.name, self.email, self.sign_in_count)
    }
}

//  元组结构体

struct Color(u32, u32, u32);

// 没有任何字段的类单元结构体
struct UnitLikeStructs;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 { // 使用引用不会占用所有权。
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
// 可以定义多个 impl 块
impl Rectangle {
    // 关联函数， 通过 Rectangle::square 调用
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        name: String::from("someone"),
        sign_in_count: 1
    };
    let user2 = User {
        email: String::from("someone2@example.com"),
        name: String::from("someone2"),
        ..user // 读取剩余的字段
    };
    println!("user : {}", user.to_string());
    println!("user2: {}", user2.to_string());

    let rect = Rectangle { width: 10, height: 20 };

    let rect2 = Rectangle {
        width: 8,
        height: 12
    };

    println!("rect is {:?}, area is {}， and rect can hold rect2: {}", rect, rect.area(), rect.can_hold(&rect2));

    println!("Square: {:?}", Rectangle::square(12));
}
