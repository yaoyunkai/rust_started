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

extern "C" {
    pub fn add_in_c(a: i32, b: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_c_function() {
        let x = 10;
        let y = 20;
        let result = unsafe { add_in_c(x, y) };
        assert_eq!(result, 30);
    }
}
