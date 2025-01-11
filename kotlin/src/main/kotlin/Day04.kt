package day04

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("04/in1")

    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Int {
    return Grid(s).xmas()
}

fun pt2(s: String): Int {
    return Grid(s).trees()
}

class Grid(s: String) {
    val lines = s.trim().lines()
    fun xmas(): Int {
        val deltas = listOf(
            listOf(Pair(0, 1), Pair(0, 2), Pair(0, 3)), // right
            listOf(Pair(0, -1), Pair(0, -2), Pair(0, -3)), // left
            listOf(Pair(-1, 0), Pair(-2, 0), Pair(-3, 0)), // up
            listOf(Pair(1, 0), Pair(2, 0), Pair(3, 0)), // down
            listOf(Pair(-1, 1), Pair(-2, 2), Pair(-3, 3)), // upright
            listOf(Pair(-1, -1), Pair(-2, -2), Pair(-3, -3)), // upleft
            listOf(Pair(1, 1), Pair(2, 2), Pair(3, 3)), // downright
            listOf(Pair(1, -1), Pair(2, -2), Pair(3, -3)), // downleft
        )
        return lines.withIndex().sumOf { (r, row) ->
            row.withIndex()
                .filter { (_, ch) -> ch == 'X' }
                .sumOf { (c, _) ->
                    deltas.count { run(r, c, it) == "MAS" }
                }
        }
    }

    fun trees(): Int {
        return lines.withIndex().sumOf { (r, row) ->
            row.withIndex().sumOf { (c, ch) ->
                when (ch) {
                    'A' -> {
                        val (ul, ur, dl, dr) = listOf(
                            get(r - 1, c - 1),
                            get(r - 1, c + 1),
                            get(r + 1, c - 1),
                            get(r + 1, c + 1),
                        )
                        when (ul to ur to dl to dr) {
                            'M' to 'S' to 'M' to 'S' -> 1
                            'S' to 'M' to 'S' to 'M' -> 1
                            'S' to 'S' to 'M' to 'M' -> 1
                            'M' to 'M' to 'S' to 'S' -> 1
                            else -> 0
                        }
                    }

                    else -> 0.toInt()
                }
            }
        }
    }

    fun run(r: Int, c: Int, ps: List<Pair<Int, Int>>): String {
        return ps.map { (dr, dc) -> get(r + dr, c + dc) }.joinToString("")
    }

    fun get(r: Int, c: Int): Char {
        return lines.getOrNull(r)?.getOrNull(c) ?: ' '
    }
}
