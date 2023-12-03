proc read_file(): seq[seq[char]] =
     let filename = "input.txt"
     var input: seq[seq[char]]
     for line in filename.lines:
          var l = cast[seq[char]](line)
          # Pad the line with 3 dots to avoid some bounds checking
          for _ in 0..2:
               l.insert('.',0)
               l.add('.')
          input.add(l)
     return input

proc is_digit(c:char): bool =
     return c >= '0' and c <= '9'

proc parse_int(c:char): int =
     return int(c) - int('0')

proc part1(): int =
     var sum = 0
     var input = read_file()
     for i in 0..<input.len:
          var j  = 0
          while j < input[i].len:
               if is_digit(input[i][j]):
                    var has_symbol = false
                    var n = 0
                    for _ in 0..2:
                         n *= 10
                         n += parse_int(input[i][j])
                         if i > 0:
                              for k in j-1..j+1:
                                   if not is_digit(input[i-1][k]) and input[i-1][k] != '.':
                                        has_symbol = true
                         if i < input.len-1:
                              for k in j-1..j+1:
                                   if not is_digit(input[i+1][k]) and input[i+1][k] != '.':
                                        has_symbol = true
                         if not is_digit(input[i][j-1]) and input[i][j-1] != '.':
                              has_symbol = true
                         if not is_digit(input[i][j+1]) and input[i][j+1] != '.':
                              has_symbol = true
                         j = j + 1
                         if not is_digit(input[i][j]): break
                    if has_symbol:
                         sum += n
               j += 1
     return sum

proc get_number(line: seq[char], i: int): int =
     var n = 0
     var j = i
     while is_digit(line[j]) and j < line.len - 1:
          j += 1
     j -= 1
     for k in [1,10,100]:
          n += parse_int(line[j]) * k
          j -= 1
          if j < 0: return n
          if not is_digit(line[j]): break
     return n

proc part2(): int =
     var sum = 0
     var input = read_file()
     for i in 0..<input.len:
          for j in 0..<input[i].len:
               if input[i][j] != '*': continue
               var adj_count = 0
               var n = 1
               if i > 0:
                    var line = input[i-1]
                    if is_digit(line[j-1]):
                         adj_count += 1
                         n *= get_number(line, j-1)
                    elif is_digit(line[j]):
                         adj_count += 1
                         n *= get_number(line, j)
                    if not is_digit(line[j]) and is_digit(line[j+1]):
                         adj_count += 1
                         n *= get_number(line, j+1)
               if i < input.len-1:
                    var line = input[i+1]
                    if is_digit(line[j-1]):
                         adj_count += 1
                         n *= get_number(line, j-1)
                    elif is_digit(line[j]):
                         adj_count += 1
                         n *= get_number(line, j)
                    if not is_digit(line[j]) and is_digit(line[j+1]):
                         adj_count += 1
                         n *= get_number(line, j+1)
               if is_digit(input[i][j-1]):
                    adj_count += 1
                    n *= get_number(input[i], j-1)
               if is_digit(input[i][j+1]):
                    adj_count += 1
                    n *= get_number(input[i], j+1)
               if adj_count == 2:
                    sum += n
     return sum

proc main() =
     var sum = part1()
     echo "Part 1: ", sum
     sum = part2()
     echo "Part 2: ", sum

when isMainModule: main()
