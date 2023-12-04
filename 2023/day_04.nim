import strutils
import sequtils
import sugar

proc solve(): (int,int) =
     let filename = "input.txt"
     var (part1,part2) = (0,0)
     var lines = filename.readFile()
                         .splitLines()
                         .filter(str => not str.isEmptyOrWhitespace())
     var copies = repeat(1,lines.len)
     for i in 0..lines.high:
          var l = lines[i]
          var sep = l.find(':')
          l.delete(0..sep)
          var nums = l.split('|').map(e => e.split(' ')
                                            .filter(str => not str.isEmptyOrWhitespace())
                                            .map(str => parseInt(str)))
          var winner_nums = nums[0]
          var my_nums = nums[1]
          var points = 0
          var n_matches = 0
          for n in my_nums:
               if n in winner_nums:
                    points = if points == 0: 1 else: points * 2
                    n_matches += 1
          part1 += points
          for _ in 0..<copies[i]:
               for j in i+1..i+n_matches:
                    if j > lines.high: break
                    copies[j] += 1
     for n in copies:
          part2 += n
     return (part1,part2)

proc main() =
     var (part1,part2) = solve()
     echo "Part 1: ", part1
     echo "Part 2: ", part2

when isMainModule: main()
