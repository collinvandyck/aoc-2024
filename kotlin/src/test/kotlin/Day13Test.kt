import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day13Test {
    val in1 = readFixture("13/in1")

    @Test
    fun testPt1() {
        assertEquals(35255, day13.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(87582154060429, day13.pt2(in1))
    }
}

