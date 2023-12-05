import strutils
import sequtils
import sugar

type seed_map = seq[tuple[dst, src, length: int]]

proc find_location(maps: seq[seed_map], seed_range: int, width: int = 1, depth: int = 0): int =
     if depth == maps.len: return seed_range
     var start = seed_range
     for rule in maps[depth]:
          if start >= rule.src and start < rule.src + rule.length:
               var offset = start - rule.src
               start = rule.dst + offset
               if width + offset <= rule.length:
                    return maps.find_location(start, width, depth + 1)
               else:
                    var w = rule.length - offset
                    var ret1 = maps.find_location(start, w, depth + 1)
                    start = seed_range + w
                    var ret2 = maps.find_location(start, width - w, depth)
                    return min(ret1, ret2)
     return maps.find_location(seed_range,width,depth+1)

proc solve(): (int,int) =
     let filename = "input.txt"
     var maps: seq[seed_map]
     var lines = filename.readFile().splitLines()
     var seeds_line = lines[0].split(' ')
     var seeds = seeds_line[1..seeds_line.high].map(e => parseInt(e))
     var curr_map: seed_map
     var i = 3
     while i < lines.len:
          var line = lines[i]
          if line.isEmptyOrWhitespace():
               maps.add(curr_map)
               curr_map = @[]
               i += 1
          else:
               var nums = line.split(' ').map(e => parseInt(e))
               curr_map.add((nums[0],nums[1],nums[2]))
          i += 1
     var part1 = maps.find_location(seeds[0])
     for seed in seeds[1..seeds.high]:
          var s = maps.find_location(seed)
          if s < part1: part1 = s
     var part2 = maps.find_location(seeds[0],seeds[1])
     for i in countup(0,seeds.high,2):
          var s = maps.find_location(seeds[i],seeds[i+1])
          if s < part2: part2 = s
     return (part1,part2)

proc main() =
     var (part1,part2) = solve()
     echo "Part 1: ", part1
     echo "Part 2: ", part2

when isMainModule: main()
