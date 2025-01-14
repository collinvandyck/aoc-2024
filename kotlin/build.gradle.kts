plugins {
    kotlin("jvm") version "2.1.0"
    id("org.jetbrains.kotlinx.benchmark") version "0.4.13"
    kotlin("plugin.allopen") version "2.0.20"
}

group = "org.example"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlinx:kotlinx-benchmark-runtime:0.4.13")
    testImplementation(kotlin("test"))
}

tasks.test {
    useJUnitPlatform()
}

kotlin {
}

benchmark {
    targets {
        register("test")
        register("main")
    }
}

allOpen {
    annotation("org.openjdk.jmh.annotations.State")
}

val profilerLib = "./profiler/async-profiler/lib/libasyncProfiler.dylib"
val profilerOut = "./profiler/profiles"

tasks {
    fun registerDay(day: String) {
        register<Test>("test-$day") {
            group = "tests"
            description = "Test Day $day"
            classpath = sourceSets["test"].runtimeClasspath
            useJUnitPlatform()
            filter {
                includeTestsMatching("*${day}*")
            }
        }
        register<JavaExec>(day) {
            group = "day"
            description = "Runs Day $day"
            classpath = sourceSets["main"].runtimeClasspath
            mainClass.set("day${day}.Day${day}Kt")  // Kotlin converts Day01.kt to Day01Kt
        }
        register<JavaExec>("day${day}ProfileJfr") {
            group = "profile"
            description = "Profile Day $day JFR"
            classpath = sourceSets["main"].runtimeClasspath
            mainClass.set("day$day.Day${day}Kt")
            jvmArgs = listOf(
                "-agentpath:$profilerLib=start,jfr,event=cpu,file=$profilerOut/$day.jfr"
            )
        }
        register<JavaExec>("day${day}ProfileHtml") {
            group = "profile"
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
    (1..20).forEach { day ->
        registerDay("%02d".format(day))
    }
}
