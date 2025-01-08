package day01

import utils.readFixture
import utils.timed
import kotlin.math.absoluteValue

fun main() {
    utils.init()
    val in1 = readFixture("01/in1")
    println("pt1: ${timed { pt1(in1) }}")
    println("pt2: ${timed { pt2(in1) }}")
}

val WS_REGEX = "\\s+".toRegex()

fun pt1(s: String): Int {
    return s.trim().lines()
        .map { line ->
            val (p1, p2) = line.trim().split(WS_REGEX)
            p1.toInt() to p2.toInt()
        }.unzip()
        .let { (a, b) -> a.sorted() to b.sorted() }
        .let { (a, b) -> a.zip(b) }
        .sumOf { (a, b) -> (a - b).absoluteValue }
}

fun pt2(s: String): Int {
    return s.trim().lines()
        .map { line ->
            val (p1, p2) = line.trim().split(WS_REGEX)
            p1.toInt() to p2.toInt()
        }
        .unzip()
        .let { (a, b) -> a.sorted() to b.sorted() }
        .let { (a, b) ->
            val counts = b.groupingBy { it }.eachCount()
            a.sumOf { x -> x * (counts[x] ?: 0) }
        }
}

