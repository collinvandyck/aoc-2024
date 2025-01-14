package utils

class C3 internal constructor()

fun c3Test() {
    val x = C3()
}

class C2 {
    var foobar: Int = 5

    constructor(foo: String)
    constructor(i: Int) : this(i.toString())
    constructor(i: Int, xs: MutableList<Int>) : this(i) {
        foobar = 6
        xs.add(i)
    }
}

class C1(s: String) {
    val chs = s.lines().flatMap { it.asSequence() }
}

class Constructor {
    val foo: String = "bar"

    init {
        println("first init block")
    }

    val x = "Foo".also { println("val x is $it") }

    init {
        println("second init block")
    }

    init {
        println("third init block")
        println("foo: $foo")
        println("x: $x")
    }

    companion object {}
}

fun main() {
    Constructor()
    val res = html {
        body {
            marquee(true)
        }
    }
    println(res)
}

fun html(init: HTML.() -> Unit): HTML {
    val html = HTML()
    html.init()
    return html
}

class HTML {
    val body = Body()
    fun body(init: Body.() -> Unit) {
        body.init()
    }

    override fun toString(): String = "body: $body"
}

class Body {
    var marquee: Boolean = false
    fun marquee(b: Boolean) {
        println("party hard? $b")
    }

    override fun toString(): String = "marquee: $marquee"
}