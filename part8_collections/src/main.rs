/*

Vector

String

Map


 */
mod vector_usage;

mod string_usage;


mod redis_usage;

fn main() {
    // vector_usage::run_vector();    
    // vector_usage::run_vector_3();
    // vector_usage::use_vector_with_enum();

    string_usage::run_with_string();

    // let ret = redis_usage::fetch_an_integer().expect("get error");
    // println!("the result from redis is {ret}");
}
