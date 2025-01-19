import day21.Code
import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day21Test {
    val ex1 = readFixture("21/ex1")
    val in1 = readFixture("21/in1")

    @Test
    fun testEx1() {
        assertEquals(126384, day21.pt1(ex1))
    }

    @Test
    fun testPt1() {
        assertEquals(42, day21.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(49931782, day21.pt2(in1))
    }

    @Test
    fun testNumeric() {
        assertEquals(29, Code("029A").numeric())
        assertEquals(189, Code("189A").numeric())
        assertEquals(109, Code("109A").numeric())
    }

}

