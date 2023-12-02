import strscans
import strutils

const
     red = 12
     green = 13
     blue = 14

proc parse_line(line: string): (bool,int,int,int) =
     var (minR, minG, minB) = (0,0,0)
     var valid_game = true
     var game_turns = line.split(':')[1].split(';')
     for turn in game_turns:
          var (r,g,b) = (0,0,0)
          for handfull in turn.split(','):
               var n: int
               var color: string
               discard scanf(handfull, "$s$i $+", n, color)
               case color:
                    of "red": r += n
                    of "green": g += n
                    of "blue": b += n
                    else: discard
               if (r > red or g > green or b > blue):
                    valid_game = false
               if (r > minR): minR = r
               if (g > minG): minG = g
               if (b > minB): minB = b
     return (valid_game,minR,minG,minB)

proc main() =
     let filename = "input.txt"
     var part1 = 0
     var part2 = 0
     for line in filename.lines:
          var game_id: int
          discard scanf(line, "Game $i:", game_id)
          let (v,r,g,b) = parse_line(line)
          if v: part1 += game_id
          part2 += r * g * b
     echo "Part 1: " , part1
     echo "Part 2: " , part2

when isMainModule: main()
