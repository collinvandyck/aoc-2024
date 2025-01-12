package day10

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("10/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Grid.parse(s).let { grid ->
    grid.trailheads().sumOf { th ->
        buildSet {
            fun findit(stack: MutableList<Tile>) {
                val cur = stack.last()
                if (cur.isEnd) {
                    add(cur)
                    return
                }
                return Dir.entries
                    .mapNotNull { grid[cur.pt + it.delta] }
                    .filter { it.ch == cur.ch + 1 }
                    .filter { !stack.contains(it) }
                    .forEach { next ->
                        stack.add(next)
                        findit(stack)
                        stack.removeLast()
                    }
            }
            findit(mutableListOf(th))
        }.size
    }
}

fun pt2(s: String) = Grid.parse(s).let { grid ->
    grid.trailheads().sumOf { th ->
        fun findit(stack: MutableList<Tile>): Int {
            val cur = stack.last()
            if (cur.isEnd) return 1
            return Dir.entries
                .mapNotNull { grid[cur.pt + it.delta] }
                .filter { it.ch == cur.ch + 1 }
                .filter { !stack.contains(it) }
                .sumOf { next ->
                    stack.add(next)
                    findit(stack).also { stack.removeLast() }
                }
        }
        findit(mutableListOf(th))
    }
}

data class Tile(val pt: Point, val ch: Char) {
    val isEnd = ch == '9'
    override fun toString(): String {
        return "(${pt.row},${pt.col}:$ch)"
    }
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(pt: Point) = Point(row + pt.row, col + pt.col)
}

enum class Dir(val delta: Point) {
    Up(Point(-1, 0)), Left(Point(0, -1)), Down(Point(1, 0)), Right(Point(0, 1));
}

class Grid(val tiles: List<List<Tile>>) {
    operator fun get(pt: Point): Tile? = tiles.getOrNull(pt.row)?.getOrNull(pt.col)

    fun trailheads() = tiles.asSequence()
        .flatMap { it.asSequence() }
        .filter { it.ch == '0' }

    companion object {
        fun parse(s: String): Grid {
            return Grid(s.trim().lineSequence().mapIndexed { row, l ->
                l.asSequence().mapIndexed { col, ch ->
                    Tile(Point(row, col), ch)
                }.toList()
            }.toList())
        }
    }
}
