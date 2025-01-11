import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day02Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("02/in1")
        assertEquals(639, day02.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("02/in1")
        assertEquals(674, day02.pt2(ex1))
    }
}

