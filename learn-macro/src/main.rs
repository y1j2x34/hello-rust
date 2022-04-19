// #![feature(trace_macros)]
// trace_macros!(true);


macro_rules! test {
    [$($abc: expr), *] => {
        $(
            println!("{:?}", $abc);
        )*
    }
}

macro_rules! map {
    ( $(($k: expr) => $v: expr), * ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map

        }
    }
}

macro_rules! vec {
    // ($($e: expr), *) => {{ // vec![1,2,3]
    //     let mut v = std::vec![];
    //     $(v.push($e);)*
    //     v
    // }}

    ($($e: expr,)*) => {{ // vec![1,2,3,]
        let mut v = std::vec![];
        $(v.push($e);)*
        v
    }}
}

macro_rules! using_a {
    ($e: expr) => {
        {
            let a = 42;
            $e
        }
    }
}

fn main() {
    // util::helped!();
    let v = vec![1,2,3,];
    println!("{:?}", v);

    let a = 12;
    let x = using_a!(a / 2); // 没有 let a = 12; 将编译不过
    println!("{:?}", x)

    // test!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    // let map: std::collections::HashMap<i32, i32> = map!{
    //     (1) => 2,
    //     (2) => 3,
    //     (3) => 5,
    //     (4) => 7
    // };
    // println!("{:?}", map);
}


// macro_rules! test_syntax [
//     [] => {
//
//     };
//     () => {};
//     {} => ();
//     {} => [];
//     (([$e: expr]+),$($a: ident), *) => {
//
//     }
// ];
