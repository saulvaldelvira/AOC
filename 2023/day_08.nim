import strutils
import strscans
import tables
import sugar
import math

var pattern: string
var map = initTable[string, tuple[l,r: string]]()

proc solve_path(start: string, end_cond: proc(s:string):bool): int =
    var nsteps = 0
    var current = start
    while not end_cond(current):
        var currmap = map[current]
        case pattern[nsteps mod pattern.len]:
        of 'R': current = currmap.r
        of 'L': current = currmap.l
        else: discard
        nsteps += 1
    return nsteps

proc solve(): (int,int) =
    let filename = "input.txt"
    var lines = filename.readFile().splitLines()
    pattern = lines[0]
    var p2_start: seq[string]
    for i in 2..lines.high:
        var line = lines[i]
        var key: string
        var lvalue, rvalue: string
        if scanf(line, "$+ = ($+, $+)", key, lvalue, rvalue):
            map[key] = (lvalue,rvalue)
            if key[2] == 'A':
                p2_start.add(key)
    var part1 = solve_path("AAA", s => s.cmp("ZZZ") == 0);
    var paths: seq[int]
    for start in p2_start:
        var p = start.solve_path(e => e[2] == 'Z')
        paths.add(p)
    var part2 = paths.lcm()
    return (part1,part2)

proc main() =
    var (part1,part2) = solve()
    echo "Part 1: ", part1
    echo "Part 2: ", part2

when isMainModule: main()
