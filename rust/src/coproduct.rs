pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn factorizer<A, B, C, F, G>(f: F, g: G) -> impl Fn(Either<A, B>) -> C
  where F: Fn(A) -> C,
        G: Fn(B) -> C,
{
    move |x| {
        match x {
            Either::Left(a)  => f(a),
            Either::Right(b) => g(b),
        }
    }
}