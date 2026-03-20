/*

use iterator

*/

fn use_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
}

fn use_sum_in_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("total is {total}");

    // 产生其他迭代器
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2 = v1.iter().map(|x| x + 1);

    for val in v2 {
        println!("Got: {val}");
    }
}

pub fn foo() {
    hello_world::print("使用迭代器", use_iterator);
}
