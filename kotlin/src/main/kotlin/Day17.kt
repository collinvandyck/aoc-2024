package day17

import utils.readFixture
import utils.timed
import kotlin.math.pow

fun main() {
    val in1 = readFixture("17/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Machine(s).run()

fun pt2(s: String) = 0

data class Register(var value: Long)

@JvmInline
value class Opcode(val value: Byte) {
    companion object {
        val adv = Opcode(0)
        val bxl = Opcode(1)
        val bst = Opcode(2)
        val jnz = Opcode(3)
        val bxc = Opcode(4)
        val out = Opcode(5)
        val bdv = Opcode(6)
        val cdv = Opcode(7)
    }
}

class Machine(s: String) {
    var a: Register
    var b: Register
    var c: Register
    val opcodes: List<Opcode>
    val output = mutableListOf<Long>()
    var pos = 0

    fun run(): String {
        while (pos <= opcodes.lastIndex) {
            val (opcode, operand) = opcodes[pos] to opcodes[pos + 1]
            when (opcode) {
                Opcode.adv -> a.value = a.value / 2.0.pow(combo(operand).toDouble()).toLong()
                Opcode.bxl -> b.value = xor(b.value, operand.value.toLong())
                Opcode.bst -> b.value = combo(operand) % 8
                Opcode.jnz -> if (a.value != 0L) {
                    pos = operand.value.toInt(); continue
                }

                Opcode.bxc -> b.value = xor(b.value, c.value)
                Opcode.out -> output.add(combo(operand) % 8)
                Opcode.bdv -> b.value = a.value / 2.0.pow(combo(operand).toDouble()).toLong()
                Opcode.cdv -> c.value = a.value / 2.0.pow(combo(operand).toDouble()).toLong()
                else -> error("invalid opcode: $opcode")
            }
            pos += 2
        }
        return output.joinToString(",")
    }

    fun xor(v1: Long, v2: Long): Long {
        return v1 xor v2
    }

    fun combo(op: Opcode): Long {
        val opl = op.value.toLong()
        return when (opl) {
            in 0L..3L -> opl
            4L -> a.value
            5L -> b.value
            6L -> c.value
            else -> error("invalid combo: $op")
        }
    }

    init {
        val re = Regex("""\d+""")
        val parseReg = { r: String ->
            re.find(r)
                ?.let { Register(it.value.toLong()) }
                ?: error("invalid reg: $r")
        }
        val lines = s.trim().lines()
        a = parseReg(lines[0])
        b = parseReg(lines[1])
        c = parseReg(lines[2])
        opcodes = re.findAll(lines.get(4))
            .map { Opcode(it.value.toByte()) }
            .toList()
    }

}
