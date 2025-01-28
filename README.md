# Unexpected Behavior Accessing Vec Elements After Resizing in Rust

This repository demonstrates a potential issue in Rust when accessing elements of a `Vec` by index after its size has been modified.  Due to how Rust's `Vec` manages memory, elements may be reallocated, invalidating previous index access.

The `bug.rs` file contains the problematic code, which attempts to access elements by index after pushing new elements into the vector. This can lead to unexpected results, possibly panics or incorrect values.

The `bugSolution.rs` file shows a safer and more reliable way to handle this scenario, demonstrating how to properly manage and access the elements in a `Vec`.