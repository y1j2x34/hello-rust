
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

// 多种类型的枚举项
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}



macro_rules! curriedAdd {
    () => {
        fn r() -> i32 { 0 }
        r
    };
    ($($arg:tt),+) => {
        {
            // let mut len = 1;
            // fn count<T>(arg: T, len: i32) -> i32 {
            //     len + 1
            // }
            // $(
            //     len = count($arg, len);
            // )*

            |n: i32| -> i32 {
                let mut sum = 0;
                $(
                    sum = sum + $arg;
                )*
                return sum + n;
            }
        }
    }
}


fn main() {
    let localhost = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("{:?}", localhost);
    println!("{:?}", loopback);

    println!("curriedAdd!(1)(2) = {}", curriedAdd!(1)(2));
    println!("curriedAdd!(1, 2)(2) = {}", curriedAdd!(1, 2)(2));


    let x: i8 = 5;
    let y: Option<i8> = Some(6);
    
    let unwrapped_y = match y {
        Some(v) => v,
        None => 0
    };

    let sum = x + unwrapped_y;
    println!("{} + {} = {}", x, unwrapped_y, sum);


    // 模式匹配必须覆盖到所有情况
    let msg = response();
    match msg {
        Message::Move {x, y} => {
            println!("x: {}, y:{}", x, y)
        },
        Message::Quit => println!("Quit"),
        Message::ChangeColor(r,g,b) => {
            println!("rgb({},{},{})", r,g,b);
        },
        Message::Write(txt) => {
            println!("write: {}", txt);
        }
    }

    let msg = response();
    // 可以使用通配符 _ 代表其它不列出来的枚举
    match msg {
        Message::Quit => {
            println!("Quit!")
        },
        _ => {
            println!("Not quit!");
        }
    }

    // 上面这种场景使用 match 显得比较啰嗦，可以用 if let 代替

    let msg = response();

    if let Message::Move {x, y} = msg {
        println!("moved {}, {}", x, y);
    } else {
        println!("Not moved!");
    }
}

fn response() -> Message {
    Message::Move {
        x: 12,
        y: 12
    }
}