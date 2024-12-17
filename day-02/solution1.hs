import System.IO
import System.Environment

-- Solution

closeValues :: [Integer] -> Bool
closeValues [] = True
closeValues [x] = True
closeValues (x:y:ys) = diff >= 1 && diff <= 3 && closeValues (y:ys)
  where
    diff = abs(x - y)

ascending :: [Integer] -> Bool
ascending [] = True
ascending [x] = True
ascending (x:y:ys) = x < y && ascending (y:ys)

ordered :: [Integer] -> Bool
ordered [] = True
ordered [x] = True
ordered [x, y] = x /= y
ordered (x:y:ys)
  | x < y     = ascending (x:y:ys)
  | otherwise = ascending $ reverse (x:y:ys)

solution :: [[Integer]] -> Int
solution = length . filter closeValues . filter ordered

-- Parse input

parseLine :: String -> [Integer]
parseLine = map read . words

parseData :: String -> [[Integer]]
parseData = map parseLine . lines

-- Main

main :: IO()
main = do
  args <- getArgs
  fileContents <- readFile (head args)
  print $ solution $ parseData fileContents
