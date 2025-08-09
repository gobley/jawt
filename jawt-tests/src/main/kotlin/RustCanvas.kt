// Copyright (c) 2025 Gobley Contributors.

package dev.gobley.jawt.tests

import java.awt.Canvas
import java.awt.Component
import java.awt.Graphics
import java.awt.event.ComponentEvent
import java.awt.event.ComponentListener
import java.io.File
import java.nio.file.Files
import java.nio.file.StandardCopyOption
import java.util.concurrent.atomic.AtomicLong

class RustCanvas : Canvas(), AutoCloseable {
    private var renderContext: AtomicLong = AtomicLong(0L)

    init {
        addComponentListener(object : ComponentListener {
            override fun componentResized(e: ComponentEvent) {
                handleResize()
                render()
            }

            override fun componentMoved(e: ComponentEvent) {
                handleResize()
                render()
            }

            override fun componentShown(e: ComponentEvent) {
                destroy(renderContext.getAndSet(create(this@RustCanvas)))
                handleResize()
                render()
            }

            override fun componentHidden(e: ComponentEvent) {
                destroy(renderContext.getAndSet(0))
            }
        })
    }

    override fun addNotify() {
        super.addNotify()
        destroy(renderContext.getAndSet(create(this)))
    }

    override fun removeNotify() {
        destroy(renderContext.getAndSet(0))
        super.removeNotify()
    }

    private fun handleResize() {
        resize(renderContext.get(), width, height)
    }

    private fun render() {
        render(renderContext.get())
    }

    override fun paint(g: Graphics) {
        render()
    }

    override fun close() {
        destroy(renderContext.getAndSet(0))
    }

    companion object {
        init {
            @Suppress("UnsafeDynamicallyLoadedCode")
            Runtime.getRuntime().load(copyLibraryIfNeeded().absolutePath)
        }

        @JvmStatic
        private external fun create(component: Component): Long

        @JvmStatic
        private external fun render(renderContext: Long)

        @JvmStatic
        private external fun resize(renderContext: Long, width: Int, height: Int)

        @JvmStatic
        private external fun destroy(renderContext: Long)

        private fun copyLibraryIfNeeded(): File {
            return File.createTempFile("jawt_tests", getLibrarySuffix()).apply {
                val resourcePrefix = getResourcePrefix()
                val libraryName = System.mapLibraryName("jawt_tests")
                RustCanvas::class.java.getResourceAsStream(
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

        private fun getLibrarySuffix(): String {
            val osName = System.getProperty("os.name")
            return when {
                osName == "Mac OS X" -> ".dylib"
                osName == "Linux" -> ".so"
                osName.startsWith("Win") -> ".dll"
                else -> error("unsupported OS: $osName")
            }
        }
    }
}