import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day08Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("08/in1")
        assertEquals(291, day08.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("08/in1")
        assertEquals(1015, day08.pt2(ex1))
    }
}

