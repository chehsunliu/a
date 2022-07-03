module Main where

solve :: Char -> String -> Int -> Int -> Int
solve prevChar (currentChar : s) currentLength maxLength
  | prevChar == currentChar = solve currentChar s (currentLength + 1) (max maxLength (currentLength + 1))
  | otherwise = solve currentChar s 1 (max maxLength currentLength)
solve _ _ _ maxLength = maxLength

main :: IO ()
main = do
  s <- getLine
  let (x : xs) = s
  print (solve x xs 1 1)
