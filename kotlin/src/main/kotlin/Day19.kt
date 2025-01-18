package day19

import utils.readFixture
import utils.timed

fun main() {
    val in1 = readFixture("19/in1")
    repeat(1) {
        println("pt1: ${timed { pt1(in1) }}")
        println("pt2: ${timed { pt2(in1) }}")
    }
}

fun pt1(s: String) = Game.parse(s).let { game ->
    game.designs.count { possible(game.patterns, it) }
}

fun pt2(s: String) = Game.parse(s).let { game ->
    game.designs.sumOf { combos(game.patterns, it) }
}

fun combos(patterns: List<String>, design: String, cache: MutableMap<String, Long> = mutableMapOf()): Long {
    return when {
        design.isEmpty() -> 1
        else -> cache.getOrPut(design, {
            patterns
                .map { design.removePrefix(it) }
                .filter { it != design }
                .sumOf {
                    if (it.isEmpty()) 1
                    else combos(patterns, it, cache)
                }
        })
    }
}

fun possible(patterns: List<String>, design: String): Boolean {
    when {
        design.isEmpty() -> return true
        else -> {
            for (pat in patterns) {
                val rest = design.removePrefix(pat)
                if (rest == design) continue
                if (possible(patterns, rest)) {
                    return true
                }
            }
            return false
        }
    }
}

class Game(val patterns: List<String>, val designs: List<String>) {
    companion object {
        fun parse(s: String) = s.trim().lines().let { lines ->
            val patterns = lines[0].trim().split(", ")
            val designs = lines.drop(2)
            Game(patterns, designs)
        }
    }
}