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

#[allow(dead_code)]
struct Pair<A, B> {
    left: A,
    right: B,
}

#[allow(dead_code)]
impl<A, B> Pair<A, B> {
    fn bimap<C, D>(self, f: impl Fn(A) -> C, g: impl Fn(B) -> D)
        -> Pair<C, D>
    {
        Pair {
            left: f(self.left),
            right: g(self.right),
        }
    }
}

fn main() {
    println!("Rust {}!", "rocks");
}