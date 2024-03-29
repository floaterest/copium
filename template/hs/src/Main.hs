{-# OPTIONS_GHC -Wno-incomplete-patterns -Wno-unrecognised-pragmas -Wno-unused-imports -Wno-unused-top-binds #-}

{-# HLINT ignore "Use infix" #-}

import Control.Applicative ()
import Control.Arrow ()
import Control.Monad ()
import Data.Array ()
import Data.Char ()
import Data.Foldable ()
import Data.List ()
import Data.Maybe ()
import Data.Tuple (swap)
import Debug.Trace (trace)

type B = Bool
type C = Char
type I = Integer
type S = String

-- | answer Yes or No
yes :: B -> S
yes True = "Yes\n"
yes False = "No\n"

-- | read @[Integer]@
ints :: S -> [I]
ints = fmap read . words
{-# INLINE ints #-}

-- | pairwise zipWith
pairWith :: (a -> a -> b) -> [a] -> [b]
pairWith = (<*> tail) . zipWith
{-# INLINE pairWith #-}

divides :: Integral a => a -> a -> B
divides = ((0 ==) .) . flip rem
{-# INLINE divides #-}

-- | count number of elements that satisfies a condition
countWith :: (a -> B) -> [a] -> Int
countWith = (length .) . filter
{-# INLINE countWith #-}

-- | show tuple
show2 :: (Show a, Show b) => (a, b) -> S
show2 (a, b) = show a <> " " <> show b

read2 :: [S] -> (I, I)
read2 [a, b] = (read a, read b)

main :: IO ()
main = interact $ tostr . solve . parse
  where
    parse = words -- or ints
    solve = id
    tostr = show -- or show2
