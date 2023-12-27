/*

Vector

push


 */

#[allow(dead_code)]
pub fn run_vector() {
    let v11: Vec<i32> = Vec::new();
    let v22 = vec![1, 2, 3];
    let v33: Vec<i32> = vec!();

    println!("the vector is {:?}", v11);
    println!("the vector is {:?}", v22);
    println!("the vector is {:?}", v33);


    println!("use push");
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("the value is {:?}", does_not_exist);
}

#[allow(dead_code)]
pub fn run_vector_2() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];
    println!("The first element is: {first}");

    let first = v[0];
    println!("The first element is: {first}");

    v.push(6);

    // println!("The first element is: {first}");
}

#[allow(dead_code)]
pub fn run_vector_3() {
    // iter vector

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];

    // 可变引用
    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }
}


#[allow(dead_code)]
pub fn use_vector_with_enum() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("the row is {:?}", row);
}
