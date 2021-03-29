
/*
一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，
这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，
目的是定义一个实现某些目的所必需的行为的集合。
*/
trait Summary {
    fn summarize(&self) -> String;
    // trait 可以提供默认实现
    fn to_string(&self) -> String {
        self.summarize()
    }
}

struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("headline: {}, \nlocation: {}, \nauthor: {}, \ncontent: {}", self.headline, self.location, self.author, self.content)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("usermame: {}, \ncontent: {}, \nreply: {}, \nretweet: {}", self.username, self.content, self.reply, self.retweet)
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("震惊！痛心疾首！惨绝人寰！"),
        location: String::from("野外"),
        author: String::from("UC"),
        content: String::from("The night is deep,and let me tell you a horrible, poignant story that someone you loved is doing something obscene with someone else in the wild!")
    };
    println!("article:\n{{\n{}\n}}", article.summarize());
}

// trait 作为参数，
#[allow(dead_code)]
fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

#[allow(dead_code)]
fn ret_summarizable() -> impl Summary {
    Tweet {
        username: String::from("hourse"),
        content: String::from("of "),
        reply: false,
        retweet: false
    }
}

// Trait Bound 语法
#[allow(dead_code)]
fn generic_notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// 多个 trait bound
#[allow(dead_code, unused)]
fn mult_trait_bound<T: Summary + Clone, U: Clone + Summary>(t: T, u: U) -> u32 {
    0
}
// where 从句简化 trait bound
#[allow(dead_code, unused)]
fn mult_trait_bound_where<T, U>(t: T, u: U) -> u32 
    where T: Summary + Clone,
        U: Summary + Clone
{
    t.clone().summarize();
    0
}

#[allow(dead_code)]
fn largest<T>(list: &[T]) -> T
    where T: PartialOrd + Copy
{
    if list.len() == 0 {
        panic!("empty list!")
    }
    let mut largest = list[0]; // 需要实现 Copy trait 才能确定是可以存储在栈上的
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

use std::fmt::Display;

struct Pair<T> {
    first: T,
    second: T
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Self {
            first,
            second
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is first {}", self.first)
        } else {
            println!("The largest member is second {}", self.second)
        }
    }
}
impl<T:Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        format!("({}, {})", self.first, self.second)
    }
}