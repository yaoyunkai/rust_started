use std::fmt::Display;

/*

blanket implementations

impl<T: Display> ToString for T {
    // --snip--
}

 */


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

#[allow(unused)]
pub fn run_1() {
    let p1 = Pair {
        x: 12,
        y: 15,
    };
    p1.cmp_display()
}
