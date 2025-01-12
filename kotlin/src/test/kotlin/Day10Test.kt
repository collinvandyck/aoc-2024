import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day10Test {
    val ex1 = readFixture("10/ex1")
    val ex2 = readFixture("10/ex2")
    val in1 = readFixture("10/in1")

    @Test
    fun testEx2() {
        assertEquals(36, day10.pt1(ex2))
    }

    @Test
    fun testPt1() {
        assertEquals(820, day10.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(1786, day10.pt2(in1))
    }
}

