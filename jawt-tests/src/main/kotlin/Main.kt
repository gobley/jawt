// Copyright (c) 2025 Gobley Contributors.

package dev.gobley.jawt.tests

import java.awt.Frame
import java.awt.event.WindowAdapter
import java.awt.event.WindowEvent
import kotlin.system.exitProcess

fun main() {
    val rustCanvas = RustCanvas().apply {
        setSize(300, 300)
    }
    val frame = Frame("JAWT tests").apply {
        setSize(300, 300)
        add(rustCanvas)
    }
    frame.addWindowListener(object : WindowAdapter() {
        override fun windowClosing(e: WindowEvent) {
            rustCanvas.close()
            exitProcess(0)
        }
    })
    frame.isVisible = true
}