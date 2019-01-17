trait Clone {
    fn clone(&self) -> Self;
}

impl<T: Clone> Clone for Vec<T> {
    fn clone(&self) -> Vec<T> {
        let mut v = Vec::new();
        for elem in self {
            v.push(elem.clone());
        }
        return v;
    }
}

fn main() {
    println!("Hello, world!");
}
