package day11

import utils.readFixture
import utils.timed
import kotlin.math.log10
import kotlin.math.pow

fun main() {
    utils.init()
    val in1 = readFixture("11/in1")
    repeat(50) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = blinkN(s, 25)

fun pt2(s: String) = blinkN(s, 75)

fun blinkN(s: String, times: Int): Long {
    var nums = parse(s)
    repeat(times) { _ ->
        nums = buildMap {
            nums.forEach { (v, count) ->
                when (v) {
                    0L -> add(1, count)
                    else -> split(v)?.let { (n1, n2) ->
                        add(n1, count)
                        add(n2, count)
                    } ?: add(v * 2024, count)
                }
            }
        }
    }
    return nums.values.map { it.toLong() }.sum()
}

fun MutableMap<Long, Long>.add(v: Long, count: Long) {
    this.merge(v, count, Long::plus)
}

fun parse(s: String) = s.trim().split(" ")
    .map { it.toLong() }
    .groupingBy { it }
    .eachCount()
    .mapValues { it.value.toLong() }

fun split(num: Long) = numDigits(num).let { digits ->
    when (digits % 2) {
        1 -> null
        else -> run {
            val pow = 10.0.pow(digits / 2).toLong()
            num / pow to num % pow
        }
    }
}

fun numDigits(num: Long) =
    if (num == 0L) 1
    else log10(num.toDouble()).toInt() + 1