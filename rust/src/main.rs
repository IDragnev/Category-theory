use rust::writer::{
    self,
    Writer,
};

struct Plus { }

impl rust::core::Monoid<Plus> for String {
    fn neutral() -> String {
        "".to_owned()
    }

    fn product(&self, other: &String) -> String {
        self.clone() + other
    }
}

fn main() {
    let f = writer::compose(
        |x| Writer::new(x, format!("just {}...", x)),
        |y| Writer::new(y + 1, " plus 1...".to_owned()),
    );
    let w = f(2);
    println!("value = {0}, log = {1}", w.value, w.context);
}