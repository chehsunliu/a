module Main where
import Data.Int (Int64)

f :: Int64 -> [Int64] -> Int64
f x (y : ys) = max 0 (x - y) + f (max x y) ys
f _ [] = 0

main :: IO ()
main = do
  _ <- getLine

  s <- getLine
  let (y : ys) = map read (words s)

  print (f y ys)

  return ()