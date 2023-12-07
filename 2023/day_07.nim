import strutils
import sequtils
import tables
import algorithm
import sugar

proc solve(with_jokers: bool): int =
     var prices = @['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
     if with_jokers:
          prices.delete(prices.find('J'))
          prices.insert('J',0)

     proc type_of_hand(hand: string): int =
          var cards = initCountTable[char]()
          var jokers = 0
          for c in hand:
               if c == 'J' and with_jokers:
                    jokers += 1
               else: cards.inc(c)
          cards.sort()
          var v = cards.values.toSeq()
          if v.len == 0:
               v.add(jokers)
          else: v[0] += jokers
          if v.len == 1: return 7
          elif v.len == 2:
               if v[0] == 4: return 6
               else: return 5
          elif v.len == 3:
               if v[0] == 3: return 4
               else: return 3
          elif v.len == 4: return 2
          else: return 1

     proc compare_hands(hand1, hand2: string): int =
          var c1 = type_of_hand(hand1)
          var c2 = type_of_hand(hand2)
          if c1 == c2:
               for i in 0..hand1.high:
                    if hand1[i] != hand2[i]:
                         c1 = prices.find(hand1[i])
                         c2 = prices.find(hand2[i])
                         return c1 - c2
          return c1 - c2

     let filename = "input.txt"
     var turns: seq[tuple[hand:string,bid:int]]
     for line in filename.lines:
          var l = line.split(' ')
          turns.add((l[0],parseInt(l[1])))

     turns.sort((a,b) => compare_hands(a.hand, b.hand))

     var sum = 0
     var rank = 1
     for t in turns:
          sum += t.bid * rank
          rank += 1
     return sum

proc main() =
     echo "Part 1: ", solve(false)
     echo "Part 2: ", solve(true)

when isMainModule: main()
