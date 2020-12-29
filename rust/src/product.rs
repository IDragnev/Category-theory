pub fn product<A, B>(a: A, b: B) -> (A, B) {
    (a, b)
}

pub fn factorizer<A, B, C, F, G>(f: F, g: G) -> impl Fn(C) -> (A, B)
  where F: Fn(C) -> A,
        G: Fn(C) -> B,
        C: Copy,
{
    move |x| (f(x), g(x))
}