/*
Vector 的使用方式

*/

fn use_vector() {
    let _v: Vec<i32> = Vec::new();
    let v1 = vec![1, 2, 3];
    println!("vector V is {:#?}", v1);

    let mut v2 = Vec::new();

    v2.push(5);
    v2.push(6);
    v2.push(7);
    v2.push(8);

    println!("vector mut V is {:#?}", v2);
    v2.append(vec![7, 8, 9].as_mut());

    println!("vector mut V after append is {:#?}", v2);
}

fn get_value_fom_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn iter_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

// compile error
// fn vector_with_ownership() {
//     let mut v = vec![1, 2, 3, 4, 5];
//
//     let first = &v[0];
//     v.push(6);
//     println!("The first element is: {first}");
// }

pub fn foo() {
    hello_world::print("vector的使用方式", use_vector);
    hello_world::print("获取vector中的元素", get_value_fom_vector);
    hello_world::print("迭代vector中的元素", iter_vector);
}
