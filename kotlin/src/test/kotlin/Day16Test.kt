package benches

import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day16Test {
    val ex1 = readFixture("16/ex1")
    val ex2 = readFixture("16/ex2")
    val in1 = readFixture("16/in1")

    @Test
    fun testEx1() {
        assertEquals(7036, day16.pt1(ex1))
    }

    @Test
    fun testEx2() {
        assertEquals(11048, day16.pt1(ex2))
    }

    @Test
    fun testPt1() {
        assertEquals(99460, day16.pt1(in1))
    }

    @Test
    fun testEx1Pt2() {
        assertEquals(45, day16.pt2(ex1))
    }


    @Test
    fun testPt2() {
        assertEquals(500, day16.pt2(in1))
    }
}

