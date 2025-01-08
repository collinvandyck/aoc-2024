import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day01Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("01/in1")
        assertEquals(3569916, day01.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("01/in1")
        assertEquals(26407426, day01.pt2(ex1))
    }
}

