module Main where

f :: Integer -> [Integer] -> Integer
f n xs =
  let total = sum xs
   in n * (n + 1) `div` 2 - total

main :: IO ()
main = do
  s <- getLine
  let n = read s :: Integer

  s <- getLine
  let xs = map read (words s)

  print (f n xs)
  return ()
