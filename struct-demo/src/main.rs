struct User {
    username: String,
    age: u32,
    email: String,
}

//  语法糖
impl User {
    fn show_user(self: &User) {
        println!("name: {}, age: {}, email: {}", self.username, self.age, self.email);
    }
}

fn show_user(user: &User) {
    println!("name: {}, age: {}, email: {}", user.username, user.age, user.email);
}

fn main() {
    let someone = User {
        username: String::from("Mario"),
        age: 35,
        email: String::from("mario@example.com")
    };
    show_user(&someone);
    someone.show_user();
}
