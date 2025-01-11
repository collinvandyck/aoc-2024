import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day05Test {

    @Test
    fun testEx1() {
        val ex1 = readFixture("05/ex1")
        assertEquals(143, day05.pt1(ex1))
    }

    @Test
    fun testPt1() {
        val ex1 = readFixture("05/in1")
        assertEquals(5248, day05.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("05/in1")
        assertEquals(4507, day05.pt2(ex1))
    }
}

