/*

*/

pub fn run_1() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    };

    let config_max: Option<u32> = Some(3333);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("None.")
    }
}
