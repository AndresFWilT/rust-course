# Programming Rust Book

## A Tour of Rust

### Table of Contents

1. [Introduction](#introduction)
2. [Configuration](#configuration)
3. [Creating a Package](#initialization)
4. [Syntax](#syntax)
5. [Unit Testing](#testing)
6. [Ownership](#ownership)

## Introduction

This chapter makes an introduction to Rust System programming language.

## Configuration

Rust have some powerful tools for it's installation and configuration.

- **rustup**: tool for managing Rust installations with correct versioning.
- **rustc**: Rust compiler.
- **cargo**; Rust compilation manager, package manager and general-purpose tool.
- **rustdoc**: Rust documentation tool.

## Initialization

to create a new Rust Package, use:

```bash
cargo new hello_world
```

that command creates a new package directory named *'hello_world'*, ready to build a command-line executable.

if we move to the new directory created (*'hello_world'*), we can see the following package's top-level directory:

```bash
ls -al
total 8
drwxr-xr-x  4 andresfwilt  staff  128 Dec  3 19:06 .
drwxr-xr-x  4 andresfwilt  staff  128 Dec  3 19:06 ..
-rw-r--r--  1 andresfwilt  staff   82 Dec  3 19:06 Cargo.toml
drwxr-xr-x  3 andresfwilt  staff   96 Dec  3 19:06 src
```

we can see a file named: *Cargo.toml* that holds metadata for the package. This doesn't contain that much:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

if the package acquires new dependencies on other libraries, we can record them in this file and Cargo will take care of downloading them.

In other hand, on the *src* directory, Cargo has begun writing the program that is written in the **main.rs** file, that contains the text:

```rust
fn main() {
    println!("Hello, world!");
}
```

All of this defines Rust, it's the extent of the boilerplate for a new Rust program; two files, totaling thirteen lines.

To run the program, just use:

```bash
cargo run
```

to clean the generated files:

```bash
cargo clean
```

to run tests:

```bash
cargo test
```

## Syntax

to create a function use:

```rust
fn name() {}
```

to create mutable var, use:

```rust
mut x: i32
```

for unsigned integer values:

```rust
mut x: u32
```

to use macros, that are a way to generate code at compile time, that is not executed during the program lifetime:

```rust
assert![1 + 1 = 2]
```

by default, once a variable is initialized, it's value never change. Also, Rust does not require parentheses around the conditional expressions, but it does require curly braces around the statements they control.

Rust also can infer the type if the variable from how the variable is used.

On other hand, Rust return statement is a expression that is not followed by a semicolon:

```rust
{
    println("evaluating cosine");
    x.cos
}
```

The attributes are an open-ended system for marking functions and other declarations with extra information like attributes (C) or annotations (Java), usually used to control compiler warnings and code style checks, include code conditionally. Also tell's Rust how to interact with code written in other languages, and so on.

```rust
#[test]
```

## Testing

Rust has a simple support for testing built into the language, using **![test]** and **assert_eq**

sample with the gcd function:

```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3*11);
}
```

## Ownership

- Andrés Felipe Wilches Torres :speech_balloon: [Send Mail](mailto:andresfwilchestdev@gmail.com)
