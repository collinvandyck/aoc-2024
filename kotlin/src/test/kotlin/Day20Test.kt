import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day20Test {
    val ex1 = readFixture("20/ex1")
    val in1 = readFixture("20/in1")

    @Test
    fun testEx1() {
        assertEquals(10, day20.pt1(ex1, savingsThreshold = 10))
    }

    @Test
    fun testPt1() {
        assertEquals(1395, day20.pt1(in1))
    }

    @Test
    fun testEx2() {
        assertEquals(29, day20.pt2(ex1, savingsThreshold = 72))
    }

    @Test
    fun testPt2() {
        assertEquals(993178, day20.pt2(in1))
    }

}

