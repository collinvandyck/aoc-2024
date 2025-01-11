import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day03Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("03/in1")
        assertEquals(185797128, day03.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("03/in1")
        assertEquals(89798695, day03.pt2(ex1))
    }
}

