package day18

import utils.readFixture
import utils.timed
import java.util.*

fun main() {
    val in1 = readFixture("18/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(
    s: String,
    numBytes: Int = 1024,
    size: Bounds = Bounds(71, 71),
) = Game(s).dijkstra(numBytes, size).first

fun pt2(
    s: String,
    size: Bounds = Bounds(71, 71),
) = Game(s).let { game ->
    (1..game.bytes.size)
        .asSequence()
        .map { numBytes -> game.dijkstra(numBytes, size) }
        .find { it.first == 0 }
        ?.second ?: "unknown"
}

data class Bounds(val rows: Int, val cols: Int)

data class Point(val row: Int, val col: Int) {
    operator fun plus(other: Point) = Point(row + other.row, col + other.col)
    override fun toString(): String = "($row,$col)"

    fun valid(bounds: Bounds) =
        row >= 0 && col >= 0 && row < bounds.rows && col < bounds.cols
}

class Node(
    val pt: Point,
    var cost: Int,
    var prev: Node? = null,
) : Comparable<Node> {
    override fun compareTo(other: Node): Int {
        return cost - other.cost
    }
}

val DIRS = arrayOf(Point(0, 1), Point(0, -1), Point(1, 0), Point(-1, 0))

class Game(val bytes: List<Point>) {
    fun dijkstra(numBytes: Int, bounds: Bounds): Pair<Int, String> {
        val start = Point(0, 0)
        val end = Point(bounds.rows - 1, bounds.cols - 1)
        val walls = bytes.take(numBytes).toSet()
        val queue = PriorityQueue<Node>()
        val remaining = mutableSetOf<Point>()
        val nodes = mutableMapOf<Point, Node>()
        (0..<bounds.rows).forEach { row ->
            (0..<bounds.cols).map { col ->
                val pt = Point(row, col)
                val cost = if (pt == start) 0 else Int.MAX_VALUE
                val node = Node(pt, cost)
                nodes[pt] = node
                queue.add(node)
                remaining.add(pt)
            }
        }
        while (queue.isNotEmpty()) {
            val cur = queue.remove()
            if (!remaining.remove(cur.pt)) {
                error("could not remove remaining")
            }
            val score = cur.cost
            DIRS.map { dir -> dir + cur.pt }
                .filter { pt -> pt.valid(bounds) }
                .filter { pt -> !walls.contains(pt) }
                .filter { pt -> remaining.contains(pt) }
                .forEach { pt ->
                    val next = nodes[pt] ?: error("no node: $pt")
                    val alt = score + 1
                    if (alt < next.cost) {
                        if (!queue.remove(next)) {
                            error("could not find next in queue")
                        }
                        remaining.remove(next.pt)
                        next.cost = alt
                        next.prev = cur
                        queue.add(next)
                        remaining.add(next.pt)
                    }
                }

        }
        var count = 0
        var cur = nodes[end]
        while (cur != null) {
            count++
            if (cur.pt == start) {
                return count - 1 to ""
            }
            cur = cur.prev
        }
        return 0 to bytes[numBytes - 1].let { "${it.col},${it.row}" }
    }

    constructor(s: String) : this(
        s.trim().lines()
            .map { line -> line.trim().split(",") }
            .map { (row, col) -> row.toInt() to col.toInt() }
            .map { (row, col) -> Point(col, row) }
    )

}
