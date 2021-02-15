use crate::core::Monoid;

pub struct Writer<T, M, Op>
  where M: Monoid<Op>,
{
    pub value: T,
    pub context: M,
    pd: std::marker::PhantomData<Op>,
}

impl<T, M, Op> Writer<T, M, Op> 
  where M: Monoid<Op>,
{
    pub fn new(value: T, context: M) -> Self {
        Self {
            value,
            context,
            pd: std::marker::PhantomData,
        }
    }
}

pub fn ret<T, M, Op>(x: T) -> Writer<T, M, Op>
  where M: Monoid<Op>,
{
    Writer::new(x, M::neutral())
}

pub fn compose<F, G, A, B, C, M, Op>(g: G, f: F) -> impl Fn(A) -> Writer<C, M, Op>
    where
    M: Monoid<Op>,
    F: Fn(A) -> Writer<B, M, Op>,
    G: Fn(B) -> Writer<C, M, Op>,
{
    move |a| {
        let w1 = f(a);
        let w2 = g(w1.value);
        Writer::new(w2.value, M::product(&w1.context, &w2.context))
    }
}

pub fn fmap<A, B, M, Op>(w: Writer<A, M, Op>, f: impl Fn(A) -> B) -> Writer<B, M, Op> 
  where M: Monoid<Op>,
{
    let h = compose(|x| ret(f(x)), |x| x);
    h(w)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Plus { }

    impl Monoid<Plus> for String {
        fn neutral() -> String {
            "".to_owned()
        }

        fn product(&self, other: &String) -> String {
            self.clone() + other
        }
    }

    #[test]
    fn composition() {
        let f = compose(
            |y| Writer::new(y + 1, " plus 1".to_owned()),
            |x| Writer::new(x, format!("just {}", x)),
        );
        let w = f(2);

        assert!(w.value == 3);
        assert!(w.context == "just 2 plus 1");
    }

    #[test]
    fn functoriality() {
        let w = Writer::new(2, "context".to_owned());
        let w = fmap(w, |x| x.to_string());

        assert!(w.value == "2");
        assert!(w.context == "context");
    }
}