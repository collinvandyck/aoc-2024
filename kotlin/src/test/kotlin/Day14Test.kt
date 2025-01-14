import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day14Test {
    val in1 = readFixture("14/in1")

    @Test
    fun testPt1() {
        assertEquals(225521010, day14.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(7774, day14.pt2(in1))
    }

    @Test
    fun testRemainder() {
        assertEquals(5, (-5 + 10) % 10)
        assertEquals(5, ((-15 % 10) + 10) % 10)
        assertEquals(5, ((-55 % 10) + 10) % 10)
    }
}

