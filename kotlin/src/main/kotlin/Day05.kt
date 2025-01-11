package day05

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("05/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Int {
    val (rules, lines) = parse(s)
    return lines
        .map { it.split(",").map(String::toInt) }
        .filter { rules.valid(it) }
        .sumOf { it[it.size / 2] }
}

fun pt2(s: String): Int {
    val (rules, lines) = parse(s)
    return lines
        .map { it.split(",").map(String::toInt) }
        .filter { !rules.valid(it) }
        .map {
            it.sortedWith { a, b ->
                if (rules.befores[a]?.contains(b) == true) -1 else 1
            }
        }
        .sumOf { it[it.size / 2] }
}

fun parse(s: String): Pair<Rules, List<String>> {
    val lines = s.trim().lines()
    val (count, rules: Rules) = lines
        .takeWhile { it.isNotEmpty() }
        .let { it.size to Rules(it) }
    val rest = lines.drop(count + 1)
    return rules to rest
}

class Rules(s: List<String>) {
    val befores: Map<Int, List<Int>> = s
        .map { it.split('|') }
        .map { (a, b) -> a.toInt() to b.toInt() }
        .groupBy(
            keySelector = { it.first },
            valueTransform = { it.second })

    fun valid(xs: List<Int>): Boolean {
        return xs.asSequence().withIndex()
            .drop(1)
            .map { (idx, x) -> x to xs.slice(0..<idx) }
            .all { (x, xs) ->
                // x is after xs, so make sure there is no before for it.
                xs.none { befores[x]?.contains(it) ?: false }
            }
    }
}

