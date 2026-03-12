// pub fn print_hello_world() {
//     println!("Hello, world! from hello world package");
// }

pub fn print<F>(message: &str, func: F)
where
    F: Fn(),
{
    println!("\n\n----- {message} -----");
    func();
}
