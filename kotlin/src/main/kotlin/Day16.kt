package day15

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("15/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = run(s)

fun pt2(s: String) = run(s, wide = true)

fun run(s: String, wide: Boolean = false) = parse(s, wide).let { (grid, moves) ->
    moves.forEach { grid.move(it) }
    grid.gpsSum()
}

data class Point(val row: Int, val col: Int) {
    operator fun plus(dir: Dir) = this + dir.dlt
    operator fun plus(point: Point) = Point(row + point.row, col + point.col)
    override fun toString(): String = "($row,$col)"
}

data class Tile(val pt: Point, var ch: Char) {
    fun isBox() = ch in "[]O"
    fun isWall() = ch == '#'

    fun gpsCoord() = when (ch) {
        '[', 'O' -> 100 * pt.row + pt.col
        else -> 0
    }

    operator fun plus(dir: Dir) = this.copy(pt = pt + dir)
    override fun toString(): String = "$pt:$ch"
}

enum class Dir(val dlt: Point, val ch: Char) {
    Up(Point(-1, 0), '^'),
    Down(Point(1, 0), 'v'),
    Left(Point(0, -1), '<'),
    Right(Point(0, 1), '>');

    val isVertical = dlt.row != 0
    override fun toString() = ch.toString()

    companion object {
        fun from(ch: Char) = Dir.entries.find { it.ch == ch } ?: error("invalid ch: $ch")
    }
}

class Grid(val tiles: List<List<Tile>>, var robot: Point) {
    fun move(dir: Dir) {
        val tile = this[robot] ?: error("robot not loaded")
        if (checkMove(tile, dir)) {
            robot += dir
        }
    }

    fun checkMove(tile: Tile, dir: Dir): Boolean {
        return arrayOf(false, true).all { moveTile(tile, dir, it) }
    }

    fun moveTile(tile: Tile, dir: Dir, apply: Boolean): Boolean {
        val next = this[tile.pt + dir] ?: run {
            return false
        }
        if (next.isWall()) return false
        if (next.isBox()) {
            if (dir.isVertical && "[]".contains(next.ch)) {
                val nexts = bigBoxTiles(next)
                if (!nexts.all { moveTile(it, dir, apply) }) {
                    return false
                }
            } else {
                if (!moveTile(next, dir, apply)) {
                    return false
                }
            }
        }
        if (apply) {
            next.ch = tile.ch
            tile.ch = '.'
        }
        return true
    }

    fun bigBoxTiles(tile: Tile): List<Tile> {
        return when (tile.ch) {
            '[' -> listOf(tile, this[tile.pt + Dir.Right] ?: error("no right tile"))
            ']' -> listOf(this[tile.pt + Dir.Left] ?: error("no left tile"), tile)
            else -> error("invalid tile: $tile")
        }
    }

    fun gpsSum() = tiles.asSequence().flatMap { it.asSequence() }.sumOf { it.gpsCoord() }

    operator fun get(pt: Point) = tiles.getOrNull(pt.row)?.getOrNull(pt.col)

    override fun toString() = tiles
        .joinToString("\n") { ts ->
            ts.map { it.ch }.joinToString("")
        }
}

fun parse(s: String, wide: Boolean = false): Pair<Grid, List<Dir>> =
    s.trim().split(Regex("""^$""", RegexOption.MULTILINE)).let { (s1, s2) ->
        val grid = s1
            .lines()
            .mapIndexed { row, l ->
                l.flatMapIndexed { col, ch ->
                    if (wide) {
                        val (ch1, ch2) = when (ch) {
                            '#' -> '#' to '#'
                            'O' -> '[' to ']'
                            '.' -> '.' to '.'
                            '@' -> '@' to '.'
                            else -> error("invalid ch: $ch")
                        }
                        listOf(Tile(Point(row, col * 2), ch1), Tile(Point(row, col * 2 + 1), ch2))
                    } else {
                        listOf(Tile(Point(row, col), ch))
                    }
                }
            }.let { tiles ->
                val robot = tiles.asSequence().flatMap { it.asSequence() }.find { it.ch == '@' } ?: error("no robot")
                Grid(tiles, robot.pt.copy())
            }
        val moves = s2.trim()
            .lines()
            .map { l -> l.map { Dir.from(it) } }
            .flatten()
        grid to moves
    }

