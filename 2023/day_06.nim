import strutils
import sequtils
import sugar

proc solve_race(time, distance: int): int =
     var sum = 0
     for i in 1..<time:
          var time_ahead = time - i
          if i * time_ahead > distance:
               sum += 1
     return sum

proc part1(): int =
     let filename = "input.txt"
     var lines = filename.readFile()
                         .splitLines()
                         .map(l => l.split(' '))
                         .map(l => l[1..l.high])
                         .map(l => l.filter(s => not s.isEmptyOrWhitespace())
                                    .map(e => parseInt(e)))
     var times = lines[0]
     var distances = lines[1]
     var acc = 1
     for i in 0..times.high:
          acc *= solve_race(times[i], distances[i])
     return acc

proc part2(): int =
     let filename = "input.txt"
     var lines = filename.readFile()
                         .splitLines()
                         .map(l => l.split(' '))
                         .map(l => l[1..l.high])
                         .map(l => l.join)
                         .filter(l => not l.isEmptyOrWhitespace())
                         .map(l => parseInt(l))
     return solve_race(lines[0],lines[1])

proc main() =
     var part1 = part1()
     echo "Part 1: ", part1
     var part2 = part2()
     echo "Part 2: ", part2

when isMainModule: main()
