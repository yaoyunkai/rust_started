// use crate::lifetimes::test_lifetime;

mod generic_start;
mod lifetimes;
mod struct_trait_usage;
mod trait_usage;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = lifetimes::longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
