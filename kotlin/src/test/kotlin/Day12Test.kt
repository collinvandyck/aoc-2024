import day12.runs
import utils.readFixture
import kotlin.test.Test
import kotlin.test.assertEquals

class Day12Test {
    val ex1 = readFixture("12/ex1")
    val ex3 = readFixture("12/ex3")
    val ex5 = readFixture("12/ex5")
    val in1 = readFixture("12/in1")

    @Test
    fun testPt1() {
        assertEquals(1396298, day12.pt1(in1))
    }

    @Test
    fun testPt2() {
        assertEquals(853588, day12.pt2(in1))
    }

    @Test
    fun testPt2Ex3() {
        assertEquals(1206, day12.pt2(ex3))
    }

    @Test
    fun testPt2Ex5() {
        assertEquals(368, day12.pt2(ex5))
    }

    @Test
    fun testRuns() {
        assertEquals(1, listOf(1).runs())
        assertEquals(1, listOf(1, 2, 3).runs())
        assertEquals(2, listOf(1, 2, 4, 5).runs())
        assertEquals(2, listOf(2, 4, 5, 1).runs())
        assertEquals(3, listOf(2, 4, 5, 1, 9, 8).runs())
    }
}

