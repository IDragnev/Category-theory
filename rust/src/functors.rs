pub mod reader {
    pub fn fmap<R, A, B, F, G, H>(
        f: impl Fn(A) -> B,
        g: impl Fn(R) -> A,
    )
        -> impl Fn(R) -> B 
    {
        crate::core::compose(f, g)
    }
}

pub struct Identity<T>(T);

impl<T> Identity<T> {
    pub fn fmap<U>(self, f: impl Fn(T) -> U) -> Identity<U> {
        Identity(f(self.0))
    }
}