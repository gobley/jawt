import gobley.gradle.GobleyHost
import gobley.gradle.cargo.dsl.jvm

plugins {
    application
    alias(libs.plugins.kotlin.jvm)
    alias(libs.plugins.gobley.cargo)
}

cargo {
    builds.jvm {
        embedRustLibrary = (GobleyHost.current.rustTarget == rustTarget)
    }
}

application {
    mainClass = "dev.gobley.jawt.tests.MainKt"
}
