import day11.numDigits
import day11.split
import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class Day11Test {
    val in1 = readFixture("11/in1")

    @Test
    fun testEx1() {
        assertEquals(55312, day11.pt1("125 17"))
    }

    @Test
    fun testPt1() {
        assertEquals(203457, day11.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(241394363462435, day11.pt2(in1))
    }

    @Test
    fun testSplit() {
        assertNull(split(0))
        assertNull(split(9))
        assertNull(split(100))
        assertNull(split(500))
        assertNull(split(999))
        assertEquals(1L to 0L, split(10))
        assertEquals(5L to 0L, split(50))
        assertEquals(9L to 9L, split(99))
        assertEquals(20L to 24L, split(2024))
        assertEquals(20L to 4L, split(2004))
        assertEquals(200L to 4L, split(200004))
    }

    @Test
    fun testNumDigits() {
        assertEquals(numDigits(0), 1)
        assertEquals(numDigits(9), 1)
        assertEquals(numDigits(10), 2)
        assertEquals(numDigits(99), 2)
        assertEquals(numDigits(100), 3)
        assertEquals(numDigits(500), 3)
        assertEquals(numDigits(999), 3)
        assertEquals(numDigits(1000), 4)
        assertEquals(numDigits(5001), 4)
        assertEquals(numDigits(9999), 4)
        assertEquals(numDigits(1036288), 7)
    }
}

