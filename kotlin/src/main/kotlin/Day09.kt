package day09

import utils.readFixture
import utils.timed
import kotlin.math.min

fun main() {
    utils.init()
    val in1 = readFixture("09/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String): Long =
    DiskMap.parse(s).let { dm ->
        while (true) {
            val (ridx, rhs) = dm.rightBlock() ?: break
            val (lidx, lhs) = dm.leftFree() ?: break;
            if (lidx >= ridx) {
                break
            }
            val amt = min(rhs.count, lhs.count)
            val id = rhs.id
            if (amt == rhs.count) {
                rhs.free = true
                rhs.id = 0
            } else {
                rhs.count -= amt
                val last = dm.sectors.last()
                if (last.free) last.count += amt else
                    dm.sectors.addLast(Sector(id = 0, count = amt, free = true))
            }
            if (amt == lhs.count) {
                lhs.free = false
                lhs.id = id
            } else {
                lhs.count -= amt
                dm.sectors.add(lidx, Sector(id = id, count = amt, free = false))
            }
        }
        dm.checksum()
    }

fun pt2(s: String): Long =
    DiskMap.parse(s).let { dm ->
        for (ridx in dm.sectors.size - 1 downTo 0) {
            val r = dm.sectors[ridx]
            if (r.free) continue
            val (l1, l2) = dm.leftFreeRun(atLeast = r.count, maxIdx = ridx) ?: continue
            val lcount = (l1..<l2).sumOf { dm.sectors[it].count }
            (l1..<l2).forEach { dm.sectors.removeAt(it) }
            dm.sectors.add(l1, r.copy())
            if (lcount > r.count) {
                dm.sectors.add(l1 + 1, Sector(id = 0, count = lcount - r.count, free = true))
            }
            r.id = 0
            r.free = true
        }
        dm.checksum()
    }


data class Sector(var id: Int, var count: Int, var free: Boolean)

class DiskMap(var sectors: MutableList<Sector>, var lPos: Int, var rPos: Int) {

    fun leftFreeRun(atLeast: Int, maxIdx: Int): Pair<Int, Int>? {
        for (li in 0..<maxIdx) {
            val l = sectors[li]
            if (!l.free) continue
            var sum = l.count
            var ri = li + 1
            while (sum < atLeast && ri < maxIdx && ri < sectors.size) {
                val r = sectors[ri]
                if (!r.free) break
                sum += r.count
                ri++
            }
            if (sum >= atLeast) {
                return li to ri
            }
        }
        return null
    }

    fun rightBlock(): Pair<Int, Sector>? {
        while (rPos >= 0) {
            val s = sectors[rPos]
            if (!s.free && s.count > 0) {
                return rPos to s
            }
            rPos--
        }
        return null
    }

    fun leftFree(): Pair<Int, Sector>? {
        while (lPos < sectors.size) {
            val s = sectors[lPos]
            if (s.free && s.count > 0) {
                return lPos to s
            }
            lPos++
        }
        return null
    }

    fun checksum(): Long = sectors
        .asSequence()
        .flatMap { s -> (0..<s.count).asSequence().map { if (s.free) 0 else s.id } }
        .mapIndexed { i, v -> i.toLong() * v.toLong() }
        .sum()

    override fun toString(): String =
        sectors.joinToString("") { rec ->
            (0..<rec.count).joinToString("") {
                if (rec.free) "." else rec.id.toString()
            }
        }

    companion object {
        fun parse(s: String): DiskMap {
            val records = s.trim().mapIndexed { i, ch ->
                val count = ch.digitToInt()
                if (i % 2 == 0) {
                    Sector(id = i / 2, count, free = false)
                } else {
                    Sector(id = 0, count, free = true)
                }
            }.toMutableList()
            return DiskMap(records, lPos = 0, rPos = records.size - 1)
        }
    }
}
