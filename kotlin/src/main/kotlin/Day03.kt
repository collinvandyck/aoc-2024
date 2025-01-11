package day03

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("03/in1")
    println("pt1: ${timed { pt1(in1) }}")
    println("pt2: ${timed { pt2(in1) }}")
}

fun pt1(s: String): Int {
    val re = Regex("""mul\((\d+),(\d+)\)""")
    return re.findAll(s).map {
        val (a, b) = it.groupValues[1] to it.groupValues[2]
        a.toInt() * b.toInt()
    }.sum()
}

fun pt2(s: String): Int {
    data class State(val ok: Boolean = true, val sum: Int = 0)

    val re = Regex("""do\(\)|don't\(\)|mul\((\d+),(\d+)\)""")
    return re.findAll(s).fold(State(true, 0)) { (ok, sum), xs ->
        when (xs.groupValues[0]) {
            "do()" -> State(true, sum)
            "don't()" -> State(false, sum)
            else -> {
                if (ok) {
                    val (a, b) = xs.groupValues[1] to xs.groupValues[2]
                    State(ok, sum + (a.toInt() * b.toInt()))
                } else State(ok, sum)
            }
        }
    }.sum
}
