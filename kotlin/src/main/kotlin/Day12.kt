package day12

import utils.readFixture
import utils.timed

fun main() {
    utils.init()
    val in1 = readFixture("12/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Grid.parse(s).regions().sumOf { it.price() }

fun pt2(s: String) = Grid.parse(s).regions().sumOf { it.discountPrice() }

data class Region(val ch: Char, val plots: Set<Plot>) {
    fun discountPrice() = this.area() * this.sides()
    fun price() = this.area() * this.perimeter()
    fun area() = plots.size
    fun perimeter() = plots.sumOf { plot ->
        plot.borders.count { (_, _, tile) -> tile == null || tile.ch != ch }
    }

    fun sides(): Int {
        val borders = plots
            .flatMap { it.borders }
            .filter { b -> b.tile == null || b.tile.ch != ch }
        return Dir.entries.sumOf { dir ->
            val vert = dir.dlt.col == 0
            borders
                .filter { it.dir == dir }
                .groupBy { if (vert) it.pt.row else it.pt.col }
                .values
                .map { list -> list.map { if (vert) it.pt.col else it.pt.row } }
                .sumOf { it.runs() }
        }
    }
}

fun List<Int>.runs() = sorted().windowed(2).count { (a, b) -> b != a + 1 } + 1

data class Plot(val tile: Tile, val borders: List<Border>)
data class Border(val dir: Dir, val pt: Point, val tile: Tile?) {
    override fun toString() = "$dir$pt"
}

data class Tile(val pt: Point, val ch: Char)

data class Point(val row: Int, val col: Int) {
    operator fun plus(other: Dir) = this + other.dlt
    operator fun plus(other: Point) = Point(row + other.row, col + other.col)
    override fun toString() = "($row,$col)"
}

enum class Dir(val dlt: Point) {
    U(Point(-1, 0)), D(Point(1, 0)), L(Point(0, -1)), R(Point(0, 1));
}

class Grid(val tiles: List<List<Tile>>) {

    fun regions(): List<Region> {
        val visited = mutableSetOf<Point>()
        return tiles.asSequence().flatMap { it }
            .filter { !visited.contains(it.pt) }
            .map { tile ->
                flood(tile).also { region ->
                    visited.addAll(region.plots.map { it.tile.pt })
                }
            }.toList()
    }

    fun flood(tile: Tile) = Region(ch = tile.ch, plots = mutableSetOf<Plot>().also { res ->
        val queue = mutableListOf(tile)
        val visitedPts = mutableSetOf<Point>()
        while (queue.isNotEmpty()) {
            queue.removeLast().also { tile ->
                val borders = Dir.entries
                    .map { it to this[tile.pt + it] }
                    .map { (d, t) -> Border(d, tile.pt + d, t) }
                val plot = Plot(tile, borders)
                res.add(plot)
                visitedPts.add(plot.tile.pt)
                borders.mapNotNull { it.tile }
                    .filter { it.ch == tile.ch }
                    .filter { !visitedPts.contains(it.pt) }
                    .forEach { queue.add(it) }
            }
        }
    })

    operator fun get(pt: Point): Tile? =
        tiles.getOrNull(pt.row)?.getOrNull(pt.col)

    companion object {
        fun parse(s: String) = Grid(
            s.trim().lines()
                .mapIndexed { row, l ->
                    l.mapIndexed { col, ch ->
                        Tile(Point(row, col), ch)
                    }
                }
        )
    }
}