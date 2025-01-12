package day07

import utils.readFixture
import utils.timed
import kotlin.math.abs
import kotlin.math.log10
import kotlin.math.pow

fun main() {
    utils.init()
    val in1 = readFixture("07/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Long {
    return parse(s)
        .filter { it.valid(true) }
        .sumOf { it.value }
}

fun pt2(s: String): Long {
    return parse(s)
        .filter { it.valid(false) }
        .sumOf { it.value }
}

data class Equation(val value: Long, val nums: List<Long>) {
    fun valid(pt1: Boolean): Boolean {
        return isValid(pt1, nums[0], 1)
    }

    private fun isValid(pt1: Boolean, acc: Long, pos: Int): Boolean {
        if (acc > value) {
            return false
        }
        if (pos >= nums.size) {
            return acc == value
        }
        return Op.entries.asSequence()
            .filter { !pt1 || it != Op.Squash }
            .any { op ->
                isValid(pt1, op.apply(acc, nums[pos]), pos + 1)
            }
    }
}

enum class Op {
    Mul, Squash, Plus;

    fun apply(v1: Long, v2: Long) = when (this) {
        Plus -> v1 + v2
        Mul -> v1 * v2
        Squash -> {
            val digits = log10(abs(v2.toDouble())).toInt() + 1
            val pow10 = 10.toDouble().pow(digits).toLong()
            v1 * pow10 + v2
        }
    }
}

fun parse(s: String): List<Equation> {
    return s.trim().lines()
        .map { line ->
            line.split(": ")
                .let { (lhs, rhs) ->
                    Equation(lhs.toLong(), rhs.split(" ").map { it.toLong() })
                }

        }
}

