import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day09Test {
    val ex1 = readFixture("09/ex1")
    val in1 = readFixture("09/in1")

    @Test
    fun testEx1() {
        assertEquals(1928, day09.pt1(ex1))
    }

    @Test
    fun testEx2() {
        assertEquals(2858, day09.pt2(ex1))
    }

    @Test
    fun testPt1() {
        assertEquals(6401092019345, day09.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(6431472344710, day09.pt2(in1))
    }
}

