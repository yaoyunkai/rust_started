/*

ust 通过在编译时进行泛型代码的 单态化（monomorphization）

 */


#[allow(unused)]
fn largest_old(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(unused)]
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


#[allow(unused)]
fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[allow(unused)]
fn largest<T: PartialOrd>(array: &[T]) -> &T {
    let mut max = &array[0];
    for item in array {
        if item > max {
            max = item;
        }
    }
    max
}


/*

结构体中定义泛型

泛型指定限制（constraint）

*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<E> Point<E> {
    #[allow(unused)]
    pub fn x(self: &Self) -> &E {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


#[allow(unused)]
pub fn run_1() {
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_old(&number_list);
    // println!("The largest number is {}", result);
    // 
    // let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // let result = largest_old(&number_list);
    // println!("The largest number is {}", result);
    // 
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest_i32(&number_list);
    // println!("The largest number is {}", result);
    // 
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest_char(&char_list);
    // println!("The largest char is {}", result);

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}


#[allow(unused)]
pub fn run_with_struct() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("the integer is {:?}", integer);
    println!("the float is {:?}", float);

    println!("the float.x is {}", float.x());
    let result = float.distance_from_origin();
    println!("the result is {result}");
}
