///
/// a print function for print examples
///
///
/// # Arguments
///
/// * `message`: the message you want to display
/// * `func`: the function will call in **print**
///
/// returns: nothing
///
pub fn print<F>(message: &str, func: F)
where
    F: Fn(),
{
    println!("\n\n---------- {message} ----------");
    func()
}
