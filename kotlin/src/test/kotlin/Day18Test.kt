package benches

import day18.Bounds
import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day18Test {
    val ex1 = readFixture("18/ex1")
    val in1 = readFixture("18/in1")

    @Test
    fun testEx1() {
        assertEquals(22, day18.pt1(ex1, numBytes = 12, size = Bounds(7, 7)))
    }

    @Test
    fun testPt1() {
        assertEquals(334, day18.pt1(in1))
    }

    @Test
    fun testEx2() {
        assertEquals("6,1", day18.pt2(ex1, size = Bounds(7, 7)))
    }

    @Test
    fun testPt2() {
        assertEquals("20,12", day18.pt2(in1))
    }
}

