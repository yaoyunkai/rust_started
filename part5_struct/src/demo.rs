/*

Demo 

 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn test_structs() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(width1, height1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    
    println!("the rect1 is {:?}", rect1);
    println!("the rect1 is {:#?}", rect1);
}

fn area2(width: u32, height: u32) -> u32 {
    width * height
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}