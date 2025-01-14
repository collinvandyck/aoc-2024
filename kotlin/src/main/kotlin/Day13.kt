package day13

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("13/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Claw.parse(s).sumOf { it.play() }

fun pt2(s: String) = Claw.parse(s).sumOf { it.play(pt1 = false) }

data class Point(val x: Long, val y: Long) {
    operator fun plus(v: Long) = Point(x + v, y + v)
    fun toDouble() = x.toDouble() to y.toDouble()
}

data class Buttons(val a: Point, val b: Point)
data class Claw(val btns: Buttons, val prize: Point) {
    fun play(pt1: Boolean = true): Long {
        val (ax, ay) = btns.a.toDouble()
        val (bx, by) = btns.b.toDouble()
        val (gx, gy) = (if (pt1) prize else prize + 10000000000000).toDouble()
        val b = (ay * gx - ax * gy) / (ay * bx - ax * by)
        if (!epsCheck(b)) {
            return 0
        }
        val a = (gx - bx * b) / ax;
        if (!epsCheck(a)) {
            return 0
        }
        val (a2, b2) = a.toLong() to b.toLong()
        val aCost = 3L
        val bCost = 1L
        val ret = a2 * aCost + b2 * bCost
        return ret
    }

    fun epsCheck(v: Double) = (v - v.toLong().toDouble()) < 0.0001

    companion object {
        val RE = Regex("""X.*?(\d+).*?Y.*?(\d+)""")
        fun parse(s: String): List<Claw> {
            return s.trim().lines().chunked(4).map { chunk ->
                chunk.take(3).map { c ->
                    (RE.find(c) ?: error("boom")).let { grp ->
                        grp.groupValues.let { (_, x, y) ->
                            Point(x.toLong(), y.toLong())
                        }
                    }
                }.let { (a, b, prize) ->
                    Claw(Buttons(a, b), prize)
                }
            }
        }
    }
}


