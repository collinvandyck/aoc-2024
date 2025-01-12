package day08

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("08/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Int = Grid.parse(s).let { grid ->
    buildSet {
        grid.attns.values.forEach { xs ->
            combos(xs).forEach { (lhs, rhs) ->
                val slope = lhs.slope(rhs)
                listOf(lhs.point - slope, rhs.point + slope)
                    .mapNotNull { grid[it] }
                    .forEach { add(it) }
            }
        }
    }.size
}

fun pt2(s: String): Int = Grid.parse(s).let { grid ->
    buildSet {
        grid.attns.values.forEach { xs ->
            combos(xs).forEach { (lhs, rhs) ->
                val slope = lhs.slope(rhs)
                val lseq = generateSequence(rhs) { grid[it.point + slope] }
                val rseq = generateSequence(lhs) { grid[it.point - slope] }
                listOf(lseq, rseq)
                    .flatMap { it }
                    .forEach { add(it) }
            }
        }
    }.size
}

fun combos(xs: List<Tile>) =
    xs.flatMapIndexed { i, tile ->
        xs.asSequence().drop(i + 1).map { tile to it }
    }

class Grid(val tiles: List<List<Tile>>) {
    val attns = tiles.asSequence().flatten()
        .filter { it.isAntenna }
        .groupBy { it.ch }

    operator fun get(point: Point) = this.tiles.getOrNull(point.row)?.getOrNull(point.col)

    companion object {
        fun parse(s: String): Grid =
            s.trim().lines().withIndex().map { (row, l) ->
                l.withIndex().map { (col, ch) ->
                    Tile(Point(row, col), ch)
                }
            }.let(::Grid)

    }
}

data class Slope(val dy: Int, val dx: Int) {
    operator fun unaryMinus() = Slope(-dy, -dx)
}

data class Tile(val point: Point, val ch: Char) {
    val isAntenna = ch != '.'
    fun slope(other: Tile): Slope {
        val dy = other.point.row - this.point.row
        val dx = other.point.col - this.point.col
        return Slope(dy, dx)
    }
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(slope: Slope) = Point(row + slope.dy, col + slope.dx)
    operator fun minus(slope: Slope) = Point(row - slope.dy, col - slope.dx)
}