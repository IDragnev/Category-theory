use std::collections::HashMap;
use core::hash::Hash;

pub fn id<T>(x: T) -> T { x }

pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
   where
   G: Fn(A) -> B,
   F: Fn(B) -> C,
{
    move |a| { f(g(a)) }   
}

pub fn memoize<F, A, B>(f: F) -> impl FnMut(A) -> B
    where F: Fn(A) -> B,
          A: Copy + Hash + Eq,
          B: Copy,
{
    let mut answers = HashMap::new();
    move |a| {
        let b = answers.entry(a).or_insert(f(a));
        *b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_handles_id() {
        let plus1 = compose(|x| x + 1, id);

        assert!(plus1(0) == 1);    
        assert!(plus1(1) == 2);    
    }

    #[test]
    fn compose_nontrivial() {
        let plus1 = |x| x + 1;
        let plus2 = compose(plus1, plus1);

        assert!(plus2(0) == 2);    
        assert!(plus2(1) == 3);    
    }

    #[test]
    fn memoize_test() {
        let f = memoize(id);

        assert!(f(1) == 1);
        assert!(f(1) == 1);
        assert!(f(2) == 2);
    }
}