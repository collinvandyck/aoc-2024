@file:Suppress("unused", "UNUSED_PARAMETER")

package day21

import utils.readFixture
import utils.timed
import kotlin.math.max

fun main() {
    val in1 = readFixture("21/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Code.parse(s).take(1).sumOf { it.complexity() }

fun pt2(s: String): Int = TODO()

sealed class Key
data class Move(val from: Tile, val to: Tile, val dir: Dir) : Key() {
    constructor(from: Tile, to: Tile) : this(from, to, from.dirTo(to))

    override fun toString() = "{${from.ch}->${to.ch}}"
}

data object Press : Key()

data class Code(val chars: List<Char>) {
    fun shortestSequence(): Int {
        println("code: $chars")
        val numKey = NumKey()
        val chs = listOf(numKey.pos.ch) + chars
        val paths = chs.windowed(2)
            .map { (ch1, ch2) ->
                val (tSrc, tDst) = listOf(ch1, ch2).map { numKey.mustGet(it) }
                numKey.paths(tSrc, tDst)
            }
            .fold(listOf(listOf<Tile>())) { acc, list ->
                // TODO: convert each list in lists to Keys and add presses
                println("list: $list")
                acc.flatMap { pl ->
                    list.map { nl -> pl.take(max(0, pl.size - 1)) + nl }
                }
            }
        for (path in paths) {
            println(path)
        }
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
    data class PathsKey(val start: Tile, val end: Tile, val shortest: Boolean)

    val tiles: List<List<Tile>> = chs.trim().lines()
        .mapIndexed { row, line ->
            line.trim().mapIndexed { col, ch ->
                val pt = Point(row, col)
                Tile(pt, ch, this)
            }
        }
    val pos: Tile = tiles.flatten().find { it.ch == 'A' } ?: error("no start")
    val pathCache = mutableMapOf<PathsKey, List<List<Tile>>>()

    fun paths(start: Tile, end: Tile, shortest: Boolean = true): List<List<Tile>> {
        val paths = pathCache.getOrElse(PathsKey(start, end, shortest)) {
            val paths: MutableList<List<Tile>> = mutableListOf()
            val addBest = fun(path: List<Tile>) {
                if (!shortest) {
                    paths.add(path)
                    return
                }
                paths.firstOrNull()?.also { l -> if (l.size > path.size) paths.clear() }
                when {
                    paths.isEmpty() -> paths.add(path)
                    paths[0].size == path.size -> paths.add(path)
                }
            }
            val queue = ArrayDeque<MutableList<Tile>>()
            queue.add(mutableListOf(start))
            while (queue.isNotEmpty()) {
                val stack = queue.removeFirst()
                if (shortest && (paths.firstOrNull()?.size ?: stack.size) < stack.size) {
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
            pathCache[PathsKey(start, end, shortest)] = paths
            pathCache[PathsKey(end, start, shortest)] = paths.map { it.reversed() }
            paths
        }
        return paths
    }

    fun mustGet(ch: Char): Tile = this[ch] ?: error("no tile found for '$ch")

    operator fun get(ch: Char): Tile? = tiles.flatten().find { it.ch == ch }

    operator fun get(pt: Point): Tile? =
        tiles.getOrNull(pt.row)
            ?.getOrNull(pt.col)
            ?.let { t -> if (t.ch != 'X') t else null }

    override fun toString() = "tiles: $tiles\npos: $pos"
}

data class Tile(val pt: Point, val ch: Char, val graph: Graph) {
    override fun toString() = "T{$pt:$ch}"
    fun dirTo(to: Tile): Dir = when {
        pt.row < to.pt.row -> Dir.D
        pt.row > to.pt.row -> Dir.U
        pt.col > to.pt.col -> Dir.L
        pt.col < to.pt.col -> Dir.R
        else -> error("invalid dirTo from:$this to:$this")
    }
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(pt: Point) = Point(row + pt.row, col + pt.col)
    override fun toString() = "($row,$col)"
}

enum class Dir(val dlt: Point) {
    U(Point(-1, 0)), D(Point(1, 0)), L(Point(0, -1)), R(Point(0, 1));
}
