module Main where

import Prelude hiding (Functor, fmap, Monoid)
import Core

instance Monoid Int where
    neutral = 0
    product = (+)

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

instance Bifunctor (,) where
    bimap f g (x, y) = (f x, g y)

instance Bifunctor Either where
    bimap f _ (Left x)  = Left (f x)
    bimap _ g (Right y) = Right (g y)

data Identity a = Identity a

instance Functor Identity where
    fmap f (Identity x) = Identity (f x)

newtype BiComp bf fu gu a b = BiComp (bf (fu a) (gu b))

instance (Bifunctor bf, Functor fu, Functor gu) =>
    Bifunctor (BiComp bf fu gu) where
        bimap f1 f2 (BiComp x) = BiComp ((bimap (fmap f1) (fmap f2)) x)

data Tree a = Leaf a | Node (Tree a) (Tree a)

instance Functor Tree where
    fmap f (Leaf a)     = Leaf (f a)
    fmap f (Node t1 t2) = Node (fmap f t1) (fmap f t2)

instance Profunctor (->) where
    lmap = flip (.)
    rmap = (.)

main = print "haskell rocks"