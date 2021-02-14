import Prelude hiding (Functor, fmap)

class Functor f where
    fmap :: (a -> b) -> f a -> f b

instance Functor Maybe where
    fmap _ Nothing  = Nothing
    fmap f (Just x) = Just (f x)

data List a = Nil | Cons a (List a)

instance Functor List where
    fmap _ Nil = Nil
    fmap f (Cons x t) = Cons (f x) (fmap f t)

instance Functor ((->) r) where
    fmap = (.)

data Const c a = Const c

instance Functor (Const c) where
    fmap _ (Const v) = Const v 