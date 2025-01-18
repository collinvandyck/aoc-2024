package benches

import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day17Test {
    val in1 = readFixture("17/in1")

    @Test
    fun testPt1() {
        assertEquals("2,1,3,0,5,2,3,7,1", day17.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(42, day17.pt2(in1))
    }
}

