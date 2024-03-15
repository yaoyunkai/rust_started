// use crate::lifetimes::test_lifetime;

mod generic_start;
mod trait_usage;
mod hash_usage;
mod struct_trait_usage;
mod lifetimes;

fn main() {
    // generic_start::run_1();
    // generic_start::run_with_struct();

    // trait_usage::run_trait();
    // hash_usage::run_hash();

    // trait_usage::run_trait();
    // struct_trait_usage::run_1();

    // lifetimes::test_lifetime();
    // lifetimes::test_lifetime2();


    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = lifetimes::longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
