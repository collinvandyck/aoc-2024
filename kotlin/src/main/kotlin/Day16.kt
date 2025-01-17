package day16

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("16/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Grid(s).fastestPath().first().cost

fun pt2(s: String) = Grid(s).fastestPath()
    .flatMap { it.tiles }
    .map { it.pt }
    .toSet().size

data class Point(val row: Int, val col: Int) {
    operator fun plus(dir: Dir) = Point(row + dir.delta.row, col + dir.delta.col)
    override fun toString() = "($row,$col)"
}

data class Tile(val pt: Point, val ch: Char) {
    val isEnd = ch == 'E'
    val isWall = ch == '#'
    override fun toString() = "$pt$ch"
}

enum class Dir(val delta: Point) {
    Up(Point(-1, 0)), Down(Point(1, 0)), Left(Point(0, -1)), Right(Point(0, 1));

    fun opposite() = when (this) {
        Up -> Down
        Down -> Up
        Left -> Right
        Right -> Left
    }
}

data class TileDir(val tile: Tile, val dir: Dir) {
    val pt get() = tile.pt
}

class Trail() : Cloneable {
    val tiles = mutableListOf<TileDir>()
    var cost = 0

    constructor(td: TileDir) : this() {
        tiles.add(td)
    }

    constructor(other: Trail) : this() {
        tiles.addAll(other.tiles)
        cost = other.cost
    }

    fun move(td: TileDir, cost: Int) {
        tiles.add(td)
        this.cost += cost
    }

    fun size() = tiles.size
    fun cur() = tiles.last()
    fun done() = cur().tile.isEnd

}

class Grid(val tiles: List<List<Tile>>) {
    constructor(s: String) : this(
        s.trim().lines()
            .mapIndexed { row, line ->
                line.mapIndexed { col, ch ->
                    Tile(Point(row, col), ch)
                }
            }
    )

    fun fastestPath(): List<Trail> {
        val costs = mutableMapOf<TileDir, Int>()
        val bests = mutableListOf<Trail>()
        val registerCost = fun(td: TileDir, cost: Int): Boolean {
            val found: Int? = costs[td]
            return if (found != null && cost > found) false else {
                if (cost != found) costs[td] = cost
                true
            }
        }
        val registerBest = fun(trail: Trail) {
            when {
                bests.isEmpty() -> bests.add(trail)
                bests.first().cost == trail.cost -> bests.add(trail)
                trail.cost < bests.first().cost -> {
                    bests.clear()
                    bests.add(trail)
                }
            }
        }
        val trails = ArrayDeque<Trail>().also { it.add(Trail(start())) }
        while (trails.isNotEmpty()) {
            val trail = trails.removeFirst()
            val cur = trail.cur()
            if (trail.done()) {
                registerBest(trail)
                continue
            }
            Dir.entries
                .filter { it != cur.dir.opposite() }
                .map { dir -> TileDir(this.mustGet(cur.pt + dir), dir) }
                .filter { !it.tile.isWall }
                .also { nexts ->
                    nexts.forEachIndexed { idx, td ->
                        val cost = if (td.dir == cur.dir) 1 else 1001
                        if (registerCost(td, cost + trail.cost)) {
                            val next = if (idx == nexts.lastIndex) trail else Trail(trail)
                            next.move(td, cost)
                            trails.add(next)
                        }
                    }
                }
        }
        return bests
    }

    fun start() = TileDir(find('S'), Dir.Right)

    fun find(ch: Char) = find { it.ch == ch } ?: error("no tile '$ch' found")

    fun find(pred: (Tile) -> Boolean) = tiles.asSequence().flatMap { it.asSequence() }.find(pred)

    fun mustGet(pt: Point) = this[pt] ?: error("no tile at $pt")

    operator fun get(pt: Point) = tiles.getOrNull(pt.row)?.getOrNull(pt.col)

}
