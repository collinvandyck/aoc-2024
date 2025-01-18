package day20

import utils.readFixture
import utils.timed
import kotlin.math.absoluteValue

fun main() {
    val in1 = readFixture("20/in1")
    repeat(10) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String, savingsThreshold: Int = 100) = findCheats(s, savingsThreshold, cheatSecs = 2)

fun pt2(s: String, savingsThreshold: Int = 100) = findCheats(s, savingsThreshold, cheatSecs = 20)

fun findCheats(s: String, savingsThreshold: Int, cheatSecs: Int) = Grid.parse(s).let { grid ->
    val path = grid.findPath()
    var cheats = 0
    for (idx in 0..<path.lastIndex) {
        val tile = path[idx]
        for (tidx in idx + 1..path.lastIndex) {
            val t = path[tidx]
            val dist = t.pt.distance(tile.pt)
            if (dist !in 1..cheatSecs) continue
            val savings = (tidx - idx) - dist
            if (savings < savingsThreshold) continue
            cheats++
        }
    }
    cheats
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(o: Point) = Point(row + o.row, col + o.col)

    override fun toString() = "($row,$col)"

    fun distance(o: Point) = (row - o.row).absoluteValue + (col - o.col).absoluteValue
}

data class Tile(val pt: Point, val ch: Char) {
    override fun toString() = "$pt:$ch"
}

enum class Dir(val dlt: Point) {
    Up(Point(-1, 0)), Down(Point(1, 0)), Left(Point(0, -1)), Right(Point(0, 1));
}

class Grid(val tiles: List<Tile>, val rows: Int, val cols: Int) {

    fun findPath(): List<Tile> {
        val start = mustFind('S')
        val visited = BooleanArray(rows * cols)
        val stack = mutableListOf(start)
        mainWhile@ while (true) {
            val cur = stack.last()
            visited[idx(cur)] = true
            if (cur.ch == 'E') return stack
            for (dir in Dir.entries) {
                val nextPt = cur.pt + dir.dlt
                val next = this[nextPt] ?: continue
                if (next.ch == '#') continue
                if (visited[idx(next)]) continue
                stack.add(next)
                continue@mainWhile
            }
            stack.removeLast()
            if (stack.last().ch == 'S') return emptyList()
        }
    }

    fun idx(tile: Tile) = idx(tile.pt)
    fun idx(pt: Point) = pt.row * cols + pt.col
    fun mustFind(ch: Char) = tiles.find { it.ch == ch } ?: error("ch '$ch' not found")

    operator fun get(pt: Point) = tiles.getOrNull(idx(pt))

    companion object {
        fun parse(s: String): Grid {
            val tiles = s.trim().lines()
                .mapIndexed { row, line ->
                    line.mapIndexed { col, ch ->
                        Tile(Point(row, col), ch)
                    }
                }
            val rows = tiles.size
            val cols = tiles[0].size
            return Grid(tiles.flatten(), rows, cols)
        }
    }
}