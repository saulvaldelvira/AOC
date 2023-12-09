import strutils
import sequtils
import sugar

proc predict_next(input: seq[int]): int =
    var diff: seq[int]
    var allzero = true
    for i in 0..<input.high:
        diff.add(input[i+1] - input[i])
        if diff[i] != 0:
            allzero = false
    if allzero: return input[input.high]
    else: return input[input.high] + predict_next(diff)

proc predict_prev(input: seq[int]): int =
    var diff: seq[int]
    var allzero = true
    for i in 0..<input.high:
        diff.add(input[i] - input[i+1])
        if diff[i] != 0:
            allzero = false
    if allzero: return input[0]
    else: return input[0] + predict_prev(diff)

proc solve(): (int,int) =
    let filename = "input.txt"
    var lines = filename.readFile()
                        .splitLines()
                        .filter(l => not l.isEmptyOrWhitespace())
                        .map(l => l.split(' '))
                        .map(l => l.map(e => parseInt(e)))
    var (part1,part2) = (0,0)
    for l in lines:
        part1 += l.predict_next()
        part2 += l.predict_prev()
    return (part1,part2)

proc main() =
    var (part1,part2) = solve()
    echo "Part 1: ", part1
    echo "Part 2: ", part2

when isMainModule: main()
