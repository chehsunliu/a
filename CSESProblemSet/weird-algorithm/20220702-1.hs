module Main where

f :: Int -> Int
f n
  | even n = n `div` 2
  | otherwise = 3 * n + 1

fs :: Int -> String -> String
fs n s
  | n == 1 = s
  | otherwise =
      let n' = f n
       in fs n' (s ++ " " ++ show n')

main :: IO ()
main = do
  s <- getLine
  let n = read s :: Int
  putStrLn (fs n (show n))
  return ()
