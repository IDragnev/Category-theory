module Core where

import Prelude hiding (Functor, fmap, Monoid)

class Monoid a where
    neutral :: a
    product :: a -> a -> a

class Functor f where
    fmap :: (a -> b) -> f a -> f b

class Bifunctor f where
    bimap :: (a -> c) -> (b -> d) -> f a b -> f c d
    bimap g h = first g . second h
    first :: (a -> c) -> f a b -> f c b
    first g = bimap g id
    second :: (b -> d) -> f a b -> f a d
    second = bimap id