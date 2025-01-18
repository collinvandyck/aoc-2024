import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day19Test {
    val in1 = readFixture("19/in1")

    @Test
    fun testPt1() {
        assertEquals(265, day19.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(752461716635602, day19.pt2(in1))
    }
}

