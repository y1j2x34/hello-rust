
fn main() {
    // panic_function();
    // result_demo();
    // unwrap_demo();
    // expect_demo();
    // recoverable_errors_with_result_demo();
    recoverable_errors_with_result_demo2();
}

fn recoverable_errors_with_result_demo2() {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        // ? 结尾的问号相当于后面的 match 处理方法,可以直接将错误传播出去
        let mut f = File::open("/home/y1j2x34/Workspaces/lab/hello-world-rust/error-handling/src/main.rs")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    match read_username_from_file() {
        Ok(s) => println!("ok: {}", s),
        Err(e) => println!("error: {}", e)
    }
}

#[allow(dead_code)]
fn recoverable_errors_with_result_demo() {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("/home/y1j2x34/Workspaces/lab/hello-world-rust/error-handling/src/main.rs");
    
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),// ! 将不能处理的错误传播出去
        };
    
        let mut s = String::new();
    
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e), // ! 将不能处理的错误传播出去
        }
    }
    match read_username_from_file() {
        Ok(f) => println!("ok: {}", f.to_string()),
        Err(e) => println!("error: {}", e)
    }
}


#[allow(dead_code)]
fn unwrap_demo() {
    fn random_err() -> Result<i8, &'static str> {
        use rand::prelude::random;
        let x: i8 = random();
        if x < 0 { Ok(x) } else { Err("random error") }
    }
    // unwrap： 返回错误，则直接panic, 否则获取到`Ok`里面的值
    let random_number = random_err().unwrap();
    println!("random number: {}", random_number);
}

#[allow(dead_code)]
fn expect_demo() {
    fn random_err() -> Result<i8, &'static str> {
        use rand::prelude::random;
        let x: i8 = random();
        if x < 0 {Ok(x)} else {Err("random error")}
    }
    let random_number = random_err().expect("random error");
    println!("random number is: {}", random_number);
}

#[allow(dead_code)]
fn result_demo() {
    use std::fs::{File, read_to_string};
    use std::io::ErrorKind;
    
    let _f = match File::open("./main.rs") {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => panic!("file not exists"),
                _ => panic!("Problem opening the file {:?}", err)
            }
        }
    };
    match read_to_string("./main.rs") {
        Ok(v) => println!("{}", v),
        Err(e) => {
            panic!("Problem reading file! {}", e)
        }
    }
}

#[allow(dead_code)]
fn panic_function() {
    // RUST_BACKTRACE=1 cargo run 运行可以显示堆栈信息
    panic!("crash and burn");
}
