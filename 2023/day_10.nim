import strutils
import sequtils
import sugar
import tables

type Coords = tuple[y,x: int]

type
  GraphNode[T] = ref object
    data: T
    edges: seq[tuple[dst: GraphNode[T], w: int]] = @[]

  Graph[T] = object
    nodes: seq[GraphNode[T]] = @[]

proc add_node[T](self: var Graph[T], e: T) =
    self.nodes.add(GraphNode[T](data: e))

proc add_edge[T](self: var Graph[T], src, dst: T, weight: int) =
    var si = self.nodes.find(src)
    var di = self.nodes.find(dst)
    if si != -1 and di != -1:
        self.nodes[si].edges.add((self.nodes[di],weight))

proc `==`(a: GraphNode[Coords],b : Coords): bool =
    return b.x == a.data.x and b.y == a.data.y

proc furthest_path[T](self: var Graph[T]): int =
    let inf = 999999999999999999
    var distances = repeat(inf, self.nodes.len)
    var queue: seq[(int,int)]
    queue.add((0, 0))

    var max_dist = 0
    while queue.len > 0:
        let (dist,node) = queue.pop()
        if dist < distances[node]:
            distances[node] = dist
            for (dst,w) in self.nodes[node].edges:
                let newdist = dist + w
                queue.add((newdist,self.nodes.find(dst.data)))
    var max = 0
    for w in distances:
        if w > max and w != inf: max = w
    return max

var graph: Graph[Coords]

proc explore(lines: seq[string], si,sj: int, sdir: char) =
    var expqueue: seq[(int,int,char)]
    expqueue.add((si,sj,sdir))
    while expqueue.len > 0:
        var (i,j,dir) = expqueue.pop()
        if dir == 'U':
            var n = 1
            while lines[i-n][j] == '|':
                n += 1
            if lines[i-n][j] == 'F' or lines[i-n][j] == '7':
                graph.add_node((i-n,j))
                graph.add_edge((i,j),(i-n,j),n)
            if lines[i-n][j] == 'F':
                expqueue.add((i-n, j, 'R'))
            if lines[i-n][j] == '7':
                expqueue.add((i-n, j, 'L'))
        elif dir == 'D':
            var n = 1
            while lines[i+n][j] == '|':
                n += 1
            if lines[i+n][j] == 'L' or lines[i+n][j] == 'J':
                graph.add_node((i+n,j))
                graph.add_edge((i,j),(i+n,j),n)
            if lines[i+n][j] == 'L':
                expqueue.add((i+n,j, 'R'))
            if lines[i+n][j] == 'J':
                expqueue.add((i+n,j, 'L'))
        elif dir == 'L':
            var n = 1
            while lines[i][j-n] == '-':
                n += 1
            if lines[i][j-n] == 'L' or lines[i][j-n] == 'F':
                graph.add_node((i,j-n))
                graph.add_edge((i,j),(i,j-n),n)
            if lines[i][j-n] == 'L':
                expqueue.add((i,j-n, 'U'))
            if lines[i][j-n] == 'F':
                expqueue.add((i,j-n, 'D'))
        elif dir == 'R':
            var n = 1
            while lines[i][j+n] == '-':
                n += 1
            if lines[i][j+n] == 'J' or lines[i][j+n] == '7':
                graph.add_node((i,j+n))
                graph.add_edge((i,j),(i,j+n),n)
            if lines[i][j+n] == 'J':
                expqueue.add((i,j+n, 'U'))
            if lines[i][j+n] == '7':
                expqueue.add((i,j+n, 'D'))

proc solve(): int =
    let filename = "input.txt"
    var lines = filename.readFile()
                        .splitLines()
                        .filter(l => not l.isEmptyOrWhitespace())
    for _ in 0..1:
        lines.add(repeat('.',lines[0].len))
        lines.insert(repeat('.',lines[0].len),0)
    for i in 0..lines.high:
        for _ in 0..1:
            lines[i].add('.')
            lines[i].insert(".",0)

    var i = 0
    var j = 0
    while i <= lines.high:
        j = lines[i].find('S')
        if j != -1:
            graph.add_node((i,j))
            break
        i += 1
    lines.explore(i,j,'U')
    lines.explore(i,j,'D')
    lines.explore(i,j,'L')
    lines.explore(i,j,'R')
    return graph.furthest_path()

proc main() =
    let part1 = solve()
    echo "Part 1: ", part1 , " or ", part1 + 1, " (sorry you'll have to try both)"

when isMainModule: main()
