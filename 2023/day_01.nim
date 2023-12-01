import strutils
import algorithm
import sequtils
import sugar

proc part1(): int =
  var f = open("input.txt")
  var sum = 0
  while not f.endOfFile():
    var line = f.readLine()
    var number = ""
    for i in 0..<line.len:
      if line[i] >= '0' and line[i] <= '9':
        number.add(line[i])
        break
    for i in countdown(line.len-1,0):
      if line[i] >= '0' and line[i] <= '9':
         number.add(line[i])
         break
    sum += parseInt(number)
  f.close()
  return sum

proc part2(): int =
  proc find_in_line(line: string, nums: seq[string]): char =
     for i in 0..<line.len:
      if line[i] >= '0' and line[i] <= '9':
        return line[i]
      for j in 0..<nums.len:
        if line.continuesWith(nums[j],i):
          var c = j + 1 + int('0')
          return char(c)

  var nums = @["one","two","three","four","five","six","seven","eight","nine"]
  var nums_reversed = nums.map(s => cast[string](s.reversed()))

  var f = open("input.txt")
  var sum = 0
  while not f.endOfFile():
    var line = f.readLine()
    var number = ""

    var c = find_in_line(line,nums)
    number.add(c)

    line.reverse()

    c = find_in_line(line,nums_reversed)
    number.add(c)

    sum += parseInt(number)
  f.close()
  return sum

proc main() =
  var sum = part1()
  echo "Part 1: " , sum
  sum = part2()
  echo "Part 2: " , sum

when isMainModule: main()
