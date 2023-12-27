/*
String

字符串 slice str，它通常以被借用的形式出现，&str

 */


#[allow(dead_code)]
pub fn run_with_string() {
    let data = "initial contents";
    let s = data.to_string();

    println!("the string s is {:?}", s);
}
