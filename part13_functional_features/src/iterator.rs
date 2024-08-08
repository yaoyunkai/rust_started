/*
迭代器

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}



*/

pub fn runner1() {
    let v1 = vec![1, 2, 3];

    // lazy mode
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
