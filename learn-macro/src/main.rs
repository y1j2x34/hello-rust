// #![feature(trace_macros)]
// trace_macros!(true);
#![allow(dead_code)]
#![allow(unused_macros)]

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

macro_rules! vec_cus {
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
macro_rules! count {
    () => {0usize};
    ($h: expr $(,$tail: expr)*) => {
        1usize + count!($($tail),*)
    }
}

macro_rules! recurrence {
    [items:$ty:ty => $($inits:expr),*; ... ; $seq:ident[$idx: ident] = $recur: expr] => {{

        const INIT_COUNT: usize = count!($($inits),*);

        #[derive(Debug)]
        struct Recurrence {
            mem: [$ty; INIT_COUNT],
            pos: usize
        }
        struct ValueGetter<'a> {
            mem: &'a [$ty; INIT_COUNT],
            offset: isize
        }
        impl<'a> std::ops::Index<usize> for ValueGetter<'a> {
            type Output = $ty;
            fn index(&self, index: usize) -> &Self::Output {
                let real_index = (index as isize - self.offset) as usize;
                &self.mem[real_index]
            }
        }
        impl std::iter::Iterator for Recurrence {
            type Item = $ty;
            fn next(&mut self) -> Option<Self::Item> {
                if self.pos < INIT_COUNT {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    return Some(next_val);
                }
                let next_val = {
                    let offset: isize = self.pos as isize - INIT_COUNT as isize;
                    println!("pos: {}, init-count: {}, offset: {}", self.pos, INIT_COUNT, offset);
                    let $idx = self.pos;
                    let $seq = ValueGetter {
                        mem: &self.mem,
                        offset: offset
                    };
                    $recur
                };
                {
                    use std::mem::swap;
                    let mut swap_tmp = next_val;
                    for i in (0..self.mem.len()).rev() {
                        swap(&mut swap_tmp, &mut self.mem[i]);
                    }
                }
                self.pos += 1;
                Some(next_val)
            }
        }
        Recurrence {
            mem: [$($inits),*],
            pos: 0
        }
    }}

}

macro_rules! var {
    ($n: ident, $value: expr) => {
        let $n = $value;
        println!("{:?}", $n)
    }
}

fn main() {
    // struct A {
    //     mem: [i32; 4]
    // };
    // let mut a = A {
    //     mem: [1,2,3,4]
    // };
    // a.mem = [2, 3,4,5];
    // println!("{:?}", a.mem);

    let f = recurrence!(items:f64 => 0.0,1.0,3.0,4.0; ... ; f[n] = f[n-1] + f[n-2] + f[n-3] + f[n-4]);

    for (index, val) in f.take(2000).enumerate() {
        println!("[{}] = {}", index, val);
    }

    // var!(a, 5);
    // util::helped!();
    // let v = vec![1,2,3,];
    // println!("{:?}", v);
    //
    // let a = 12;
    // let x = using_a!(a / 2); // 没有 let a = 12; 将编译不过
    // println!("{:?}", x)

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
