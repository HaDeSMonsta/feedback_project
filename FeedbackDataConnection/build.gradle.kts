plugins {
    id("java")
}

repositories {
    mavenCentral()
}

dependencies {
}

val jarName = "FeedbackDataConnection"
val jarVersion = ""

// https://stackoverflow.com/questions/41794914/how-to-create-a-fat-jar-with-gradle-kotlin-script
tasks.jar {

    duplicatesStrategy = DuplicatesStrategy.INCLUDE

    archiveBaseName.set(jarName)
    version = jarVersion

    manifest.attributes["Main-Class"] = "Main"

    val dependencies = configurations
        .runtimeClasspath
        .get()
        .map { zipTree(it) }
    from(dependencies)
}

tasks.test {
    useJUnitPlatform()
}
