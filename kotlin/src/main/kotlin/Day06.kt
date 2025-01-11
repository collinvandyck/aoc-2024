package day06

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("06/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Int {
    return Grid.fromString(s).walk(true)
}

fun pt2(s: String): Int {
    return Grid.fromString(s).walk(false)
}

enum class Dir {
    Up, Down, Left, Right;

    companion object {
        val rotations = mapOf(
            Up to Right,
            Down to Left,
            Left to Up,
            Right to Down,
        )
    }

    fun rotate() = rotations[this] ?: error("invalid rotation")

    fun next(pt: Pt) = when (this) {
        Up -> Pt(pt.row - 1, pt.col)
        Down -> Pt(pt.row + 1, pt.col)
        Left -> Pt(pt.row, pt.col - 1)
        Right -> Pt(pt.row, pt.col + 1)
    }
}

data class Pt(val row: Int, val col: Int)
data class Tile(val pt: Pt, val ch: Char) {
    fun isWall() = ch == '#'
    fun isStart() = ch == '^'
}

class Grid(val tiles: List<List<Tile>>) {
    companion object {
        fun fromString(s: String): Grid {
            val tiles = s.trim().lines().withIndex().map { (row, line) ->
                line.withIndex().map { (col, ch) ->
                    Tile(Pt(row, col), ch)
                }
            }
            return Grid(tiles)
        }
    }

    fun walk(pt1: Boolean): Int {
        var cycles = 0
        var (cur, dir) = this.start()
        val visited = mutableMapOf<Tile, MutableSet<Dir>>()
        var step = 0
        while (true) {
            step += 1
            val tileDirs = visited.getOrPut(cur) { mutableSetOf() }
            if (!tileDirs.add(dir)) {
                break
            }
            val next = this.get(dir.next(cur.pt)) ?: break
            if (next.isWall()) {
                dir = dir.rotate()
                continue
            }
            if (!pt1 && !visited.contains(next)) {
                val v2 = visited.mapValuesTo(mutableMapOf()) { (_, dirs) -> dirs.toMutableSet() }
                if (CycleCheck(this, cur, dir, v2, next).cycle()) {
                    cycles += 1
                }
            }
            cur = next

        }
        return if (pt1) visited.size else cycles
    }

    data class CycleCheck(
        val grid: Grid,
        var cur: Tile,
        var dir: Dir,
        val visited: MutableMap<Tile, MutableSet<Dir>>,
        val wall: Tile
    ) {
        fun cycle(): Boolean {
            visited.remove(cur)
            while (true) {
                val tileDirs = visited.getOrPut(cur) { mutableSetOf() }
                if (!tileDirs.add(dir)) {
                    return true
                }
                val next = grid.get(dir.next(cur.pt)) ?: break
                if (next.isWall() || next.pt == wall.pt) {
                    dir = dir.rotate()
                    continue
                }
                cur = next
            }
            return false
        }
    }

    fun get(pt: Pt): Tile? {
        return tiles.getOrNull(pt.row)?.getOrNull(pt.col)
    }

    fun start() = tiles.asSequence()
        .flatten()
        .find { it.isStart() }
        ?.let { it to Dir.Up }
        ?: error("no start")

}

