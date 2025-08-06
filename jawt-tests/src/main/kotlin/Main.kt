// Copyright (c) 2025 Gobley Contributors.

package dev.gobley.jawt.tests

import java.awt.Frame
import java.awt.event.WindowAdapter
import java.awt.event.WindowEvent
import kotlin.system.exitProcess

fun main() {
    val frame = Frame("JAWT tests: Rust.add(5, 4) = ${Rust.add(5, 4)}")
    frame.setSize(300, 300)
    frame.isVisible = true
    frame.addWindowListener(object : WindowAdapter() {
        override fun windowClosing(e: WindowEvent) {
            exitProcess(0)
        }
    })
}