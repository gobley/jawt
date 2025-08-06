// Copyright (c) 2025 Gobley Contributors.

package dev.gobley.jawt.tests

import java.io.File
import java.nio.file.Files
import java.nio.file.StandardCopyOption

object Rust {
    init {
        @Suppress("UnsafeDynamicallyLoadedCode")
        Runtime.getRuntime().load(copyLibraryIfNeeded().absolutePath)
    }

    private fun copyLibraryIfNeeded(): File {
        return File.createTempFile("jawt_tests", "").apply {
            val resourcePrefix = getResourcePrefix()
            val libraryName = System.mapLibraryName("jawt_tests")
            Rust::class.java.getResourceAsStream(
                "/$resourcePrefix/$libraryName"
            )!!.use { inputStream ->
                Files.copy(inputStream, toPath(), StandardCopyOption.REPLACE_EXISTING)
            }
            deleteOnExit()
        }
    }

    private fun getResourcePrefix(): String {
        return StringBuilder().apply {
            val osName = System.getProperty("os.name")
            append(
                when {
                    osName == "Mac OS X" -> "darwin"
                    osName == "Linux" -> "linux"
                    osName.startsWith("Win") -> "win32"
                    else -> error("unsupported OS: $osName")
                }
            )
            append('-')
            val osArch = System.getProperty("os.arch")
            append(
                when (osArch) {
                    "x86_64", "amd64" -> "x86-64"
                    "aarch64" -> "aarch64"
                    else -> error("unknown arch: $osArch")
                }
            )
        }.toString()
    }

    @JvmStatic
    external fun add(lhs: Int, rhs: Int): Int
}