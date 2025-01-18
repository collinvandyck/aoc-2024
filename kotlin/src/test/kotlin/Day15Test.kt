import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day15Test {
    val ex11 = readFixture("15/ex11")
    val ex1 = readFixture("15/ex1")
    val in1 = readFixture("15/in1")

    @Test
    fun testEx11() {
        assertEquals(2028, day15.pt1(ex11))
    }

    @Test
    fun testEx1() {
        assertEquals(10092, day15.pt1(ex1))
    }

    @Test
    fun testPt1() {
        assertEquals(1475249, day15.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(1509724, day15.pt2(in1))
    }
}

