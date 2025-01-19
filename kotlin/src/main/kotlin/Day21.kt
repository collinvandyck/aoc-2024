package day21

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("21/in1")
    repeat(10) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Code.parse(s).take(1).sumOf { it.complexity() }

fun pt2(s: String): Int = TODO()

data class Code(val chars: List<Char>) {
    fun shortestSequence(): Int {
        val numKey = NumKey()
        val paths = numKey.shortestPaths(numKey['A']!!, numKey['5']!!)
        println("paths:\n${paths.joinToString("\n")}")
        return 0
    }

    fun complexity() = shortestSequence() * numeric()

    fun numeric() = chars
        .filter { it.isDigit() }
        .map { it.digitToInt() }
        .fold(0) { acc, i -> acc * 10 + i }

    constructor(s: String) : this(s.toList())

    companion object {
        fun parse(s: String): List<Code> = s.trim()
            .lines()
            .map { it.toList() }
            .map { Code(it) }
    }
}

class NumKey : Graph("789\n456\n123\nX0A")
class DirKey : Graph("X^A\n<v>")

open class Graph(chs: String) {
    val tiles: List<List<Tile>> = chs.trim().lines()
        .mapIndexed { row, line ->
            line.trim().mapIndexed { col, ch ->
                val pt = Point(row, col)
                Tile(pt, ch, this)
            }
        }

    val pos: Tile = tiles.flatten().find { it.ch == 'A' } ?: error("no start")
    val pathCache = mutableMapOf<Pair<Tile, Tile>, List<List<Tile>>>()

    fun shortestPaths(start: Tile, end: Tile): List<List<Tile>> {
        return pathCache.getOrPut(start to end, {
            val res: MutableList<List<Tile>> = mutableListOf()
            val addBest = fun(list: List<Tile>) {
                res.firstOrNull()?.also { l -> if (l.size > list.size) res.clear() }
                when {
                    res.isEmpty() -> res.add(list)
                    res[0].size == list.size -> res.add(list)
                }
            }
            val queue = ArrayDeque<MutableList<Tile>>()
            queue.add(mutableListOf(start))
            while (queue.isNotEmpty()) {
                val stack = queue.removeFirst()
                if ((res.firstOrNull()?.size ?: stack.size) < stack.size) {
                    continue
                }
                val cur = stack.last()
                if (cur == end) {
                    addBest(stack)
                    continue
                }
                val dirs = Dir.entries
                    .mapNotNull { this[cur.pt + it.dlt] }
                    .filter { !stack.contains(it) }
                dirs.forEachIndexed { idx, next ->
                    val ns = if (idx == dirs.lastIndex) stack else stack.toMutableList()
                    queue.addLast(ns.also { it.add(next) })
                }
            }
            res
        })
    }

    fun path(from: Char, to: Char): List<Tile> {
        TODO()
    }

    operator fun get(ch: Char): Tile? = tiles.flatten().find { it.ch == ch }

    operator fun get(pt: Point): Tile? =
        tiles.getOrNull(pt.row)
            ?.getOrNull(pt.col)
            ?.let { t -> if (t.ch != 'X') t else null }

    override fun toString() = "tiles: $tiles\npos: $pos"
}

data class Tile(val pt: Point, val ch: Char, val graph: Graph) {
    override fun toString() = "$pt:$ch"
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(pt: Point) = Point(row + pt.row, col + pt.col)
    override fun toString() = "($row,$col)"
}

enum class Dir(val dlt: Point) {
    U(Point(-1, 0)), D(Point(1, 0)), L(Point(0, -1)), R(Point(0, 1));
}
