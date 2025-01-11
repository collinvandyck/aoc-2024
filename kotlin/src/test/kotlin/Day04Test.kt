import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day04Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("04/in1")
        assertEquals(2458, day04.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("04/in1")
        assertEquals(1945, day04.pt2(ex1))
    }
}

