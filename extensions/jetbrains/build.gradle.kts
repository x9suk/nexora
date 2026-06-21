plugins {
    id("java")
    id("org.jetbrains.intellij") version "1.17.2"
}

group = "com.nexora"
version = "1.0.0"

repositories {
    mavenCentral()
}

intellij {
    version.set("2024.1")
    type.set("IC")
    plugins.set(listOf("com.intellij.java"))
}

tasks {
    patchPluginXml {
        sinceBuild.set("241")
        untilBuild.set("261.*")
    }

    compileJava {
        sourceCompatibility = JavaVersion.VERSION_17.toString()
        targetCompatibility = JavaVersion.VERSION_17.toString()
    }
}
