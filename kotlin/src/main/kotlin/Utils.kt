package utils

import java.io.File
import kotlin.system.measureNanoTime
import kotlin.time.Duration.Companion.nanoseconds

fun init() {
    Git.root
}

fun readFixture(loc: String): String {
    val root = Git.root
    val path = "$root/data/$loc"
    return File(path).readText()
}

fun <T> timed(f: () -> T): T {
    var res: T
    val nanos = measureNanoTime { res = f() }.nanoseconds
    println("> $nanos")
    return res
}

object Git {
    val root: String

    init {
        val proc = ProcessBuilder("git", "rev-parse", "--show-toplevel")
            .redirectOutput(ProcessBuilder.Redirect.PIPE)
            .redirectError(ProcessBuilder.Redirect.PIPE)
            .start()
        this.root = proc.inputStream.bufferedReader().readText().trim()
    }
}
