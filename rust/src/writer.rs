use super::core::Monoid;

pub struct Writer<T, M, Op>
  where T: Copy,
        M: Monoid<Op>,
{
    pub value: T,
    pub context: M,
    pd: std::marker::PhantomData<Op>,
}

impl<T, M, Op> Writer<T, M, Op>
  where T: Copy,
        M: Monoid<Op>
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
        T: Copy,
{
    Writer::new(x, M::neutral())
}

pub fn compose<F, G, A, B, C, M, Op>(f: F, g: G) -> impl Fn(A) -> Writer<C, M, Op>
    where
    A: Copy,
    B: Copy,
    C: Copy,
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