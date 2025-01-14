package day14

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("14/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Grid.parse(s).let { grid ->
    repeat(100) { grid.step() }
    grid.safetyFactor()
}

fun pt2(s: String) = Grid.parse(s).let { grid ->
    generateSequence(1) { it + 1 }
        .find {
            grid.step()
            grid.isTree()
        }
}


data class Point(var col: Int, var row: Int)
data class Robot(val pt: Point, val velocity: Point)
class Grid(val robots: List<Robot>, val rows: Int, val cols: Int) {

    fun isTree() = robots.map { it.pt }
        .groupBy { it.row }
        .values
        .any { pts ->
            if (pts.size <= TREE_RUN) return@any false
            data class Acc(val run: Int, val max: Int)
            (pts.asSequence()
                .map { it.col }
                .sorted()
                .windowed(2)
                .scan(1) { acc, (v1, v2) -> if (v2 - v1 == 1) acc + 1 else 1 }
                .maxOrNull() ?: 0) >= TREE_RUN
        }

    fun safetyFactor(): Int {
        val (rmid, cmid) = rows / 2 to cols / 2
        return robots
            .fold(intArrayOf(0, 0, 0, 0)) { acc, (pt, _) ->
                val (col, row) = pt
                when {
                    col < cmid && row < rmid -> acc[0]++
                    col < cmid && row > rmid -> acc[1]++
                    col > cmid && row < rmid -> acc[2]++
                    col > cmid && row > rmid -> acc[3]++
                }
                acc
            }
            .reduce(Int::times)
    }

    private fun Int.wrapMod(n: Int) = ((this % n) + n) % n

    fun step() {
        for ((pt, vel) in robots) {
            pt.row = (pt.row + vel.row).wrapMod(rows)
            pt.col = (pt.col + vel.col).wrapMod(cols)
            assert(pt.col >= 0 && pt.row >= 0)
        }
    }

    companion object {
        const val TREE_RUN = 10
        val RE = Regex("""(-?\d+),(-?\d+).*?(-?\d+),(-?\d+)""")

        fun parse(s: String) = Grid(
            rows = 103, cols = 101,
            robots = s.trim().lines().map { l ->
                val matches = RE.find(l) ?: error("boom")
                matches.groupValues.let { (_, p1, p2, v1, v2) ->
                    val pt = Point(p1.toInt(), p2.toInt())
                    assert(pt.col >= 0 && pt.row >= 0)
                    val velocity = Point(v1.toInt(), v2.toInt())
                    Robot(pt, velocity)
                }
            })

    }
}
