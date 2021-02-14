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