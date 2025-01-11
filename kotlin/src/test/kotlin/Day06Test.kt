import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day06Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("06/in1")
        assertEquals(5564, day06.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("06/in1")
        assertEquals(1976, day06.pt2(ex1))
    }
}

