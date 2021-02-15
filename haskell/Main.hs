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

main = print "haskell rocks"