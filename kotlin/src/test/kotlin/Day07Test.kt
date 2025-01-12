import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day07Test {

    @Test
    fun testPt1() {
        val ex1 = readFixture("07/in1")
        assertEquals(28730327770375, day07.pt1(ex1))
    }

    @Test
    fun testPt2() {
        val ex1 = readFixture("07/in1")
        assertEquals(424977609625985, day07.pt2(ex1))
    }
}

