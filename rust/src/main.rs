#[allow(dead_code)]
enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

#[allow(dead_code)]
fn fmap<A, B, F>(t: &Tree<A>, f: F) -> Tree<B> 
  where F: Fn(&A) -> B + Copy
{
    match t {
        Tree::Leaf(a) => Tree::Leaf(f(a)),
        Tree::Node(t1, t2) => {
            Tree::Node(
                Box::new(fmap(&t1, f)),
                Box::new(fmap(&t2, f)),
            )
        },
    }
}

fn main() {
    println!("Rust {}!", "rocks");
}