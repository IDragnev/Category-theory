pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn bimap<C, D>(self, f: impl Fn(A) -> C, g: impl Fn(B) -> D)
     -> Either<C, D>
    {
        match self {
            Either::Left(a)  => Either::Left(f(a)),
            Either::Right(b) => Either::Right(g(b)),
        }
    }
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