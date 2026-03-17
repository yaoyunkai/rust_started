/*
dict

*/
use std::collections::HashMap;

fn create_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:#?}", scores);

    let score = scores.get("Blue").copied().unwrap_or(0);
    print!("score from Blue is {}", score);
}

fn iter_hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

pub fn foo() {
    hello_world::print("Hashmap的使用", create_hashmap);
    hello_world::print("Hashmap的迭代", iter_hashmap);
}
