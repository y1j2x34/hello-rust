mod common;

use common::math::sqrt::sqrt;


fn main() {
    println!("sub: {}", common::math::sub::sub(1, 2));
    let sum = common::math::add(1,2);
    println!("sum: {}", sum);
    println!("sqrt(5): {}", sqrt(5));
}