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

tasks {
    register<JavaExec>("day01") {
        group = "advent"
        description = "Runs Day 1"
        classpath = sourceSets["main"].runtimeClasspath
        mainClass.set("day01.Day01Kt")  // Kotlin converts Day01.kt to Day01Kt
    }

    // You can add more days following the same pattern
    // register<JavaExec>("day02") {
    //     group = "advent"
    //     description = "Runs Day 2"
    //     classpath = sourceSets["main"].runtimeClasspath
    //     mainClass.set("Day02Kt")
    // }
}