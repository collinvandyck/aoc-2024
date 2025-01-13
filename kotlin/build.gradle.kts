plugins {
    kotlin("jvm") version "2.0.0"
}

group = "org.example"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

val profilerLib = "./profiler/async-profiler/lib/libasyncProfiler.dylib"
val profilerOut = "./profiler/profiles"

tasks {
    fun registerDay(day: String) {
        register<JavaExec>("day${day}") {
            group = "advent"
            description = "Runs Day $day"
            classpath = sourceSets["main"].runtimeClasspath
            mainClass.set("day${day}.Day${day}Kt")  // Kotlin converts Day01.kt to Day01Kt
        }
        register<JavaExec>("day${day}ProfileJfr") {
            group = "advent"
            description = "Profile Day $day JFR"
            classpath = sourceSets["main"].runtimeClasspath
            mainClass.set("day$day.Day${day}Kt")
            jvmArgs = listOf(
                "-agentpath:$profilerLib=start,jfr,event=cpu,file=$profilerOut/$day.jfr"
            )
        }
        register<JavaExec>("day${day}ProfileHtml") {
            group = "advent"
            description = "Profile Day $day"
            classpath = sourceSets["main"].runtimeClasspath
            mainClass.set("day$day.Day${day}Kt")
            jvmArgs = listOf(
                "-agentpath:$profilerLib=start,event=cpu,file=$profilerOut/$day.html"
            )
            doLast {
                exec {
                    commandLine("open", "$profilerOut/${day}.html")
                }
            }
        }
    }
    registerDay("09")
    registerDay("10")
    registerDay("11")
}
