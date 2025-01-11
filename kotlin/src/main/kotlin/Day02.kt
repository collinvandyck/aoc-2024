package day02

import utils.readFixture
import utils.timed
import kotlin.math.absoluteValue

fun main() {
    utils.init()
    val in1 = readFixture("02/in1")
    println("pt1: ${timed { pt1(in1) }}")
    println("pt2: ${timed { pt2(in1) }}")
}

fun pt1(s: String): Int {
    return s.trim().lineSequence()
        .map { it.split(" ").map(String::toInt) }
        .filter { isValid(it) }
        .count()
}

fun pt2(s: String): Int {
    return s.trim().lineSequence()
        .map { it.split(" ").map(String::toInt) }
        .filter {
            val others = it.indices.map { skip -> it.filterIndexed { idx, _ -> idx != skip } }
            val lists = listOf(it) + others
            lists.any { isValid(it) }
        }
        .count()
}

fun isValid(xs: List<Int>): Boolean {
    data class State(var inc: Boolean, var dec: Boolean, var dist: Boolean)
    return xs.windowed(size = 2, step = 1)
        .fold(State(true, true, true)) { acc: State, (e1, e2) ->
            acc.inc = acc.inc && e1 > e2
            acc.dec = acc.dec && e1 < e2
            acc.dist = acc.dist && (e1 - e2).absoluteValue in 1..3
            acc
        }
        .let { (inc, dec, dist) -> (inc || dec) && dist }
}

