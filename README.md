# Unexpected Iterator Behavior in Rust

This repository demonstrates a common pitfall when working with iterators in Rust: the behavior after the iterator is exhausted.

The `bug.rs` file contains code that iterates over a vector.  After the iterator has consumed all elements, it continues to return `None` rather than panicking or raising an error. This behavior, while defined, can be surprising to developers accustomed to different iterator paradigms. 

The `bugSolution.rs` file shows several ways to address this, depending on the desired behavior: checking for `None` before processing the next element, using iterator adaptors that handle exhaustion gracefully (like `collect`), or choosing different data structures better suited to your needs.