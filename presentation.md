---
title: What is Rust?
sub_title: What is this language that everyone talks about, and a look at some features?
author: Roald Strangstadstuen
theme:
  override:
    code:
      alignment: left
      background: true
---

This presentation
---

* What is Rust?
* How to do memory safety without a garbage collector
* Some cool features
* Rust goals (speed, safety, and productivity)
* unsafe
* When to use / not use Rust
* Demo time

<!-- end_slide -->

What is Rust?
---

* Rust is a general-purpose, systems-level programming language.
<!-- pause  -->
* It was started by Mozilla, with version 1.0 released in May 2015.

<!-- end_slide -->
Rust’s memory safety
---

Ownership and borrowing

* All values have one unique owner.
<!-- pause  -->
* The owner is responsible for freeing the underlying value once it goes out of scope.
<!-- pause  -->
* There are three ways values can be transferred:
  1. Transfer of ownership
  2. Immutable borrow(s)
  3. Mutable borrow

<!-- pause  -->
Lifetimes

* All owned values have a lifetime
  * The lifetime of any reference cannot outlive the lifetime of the owner

<!-- end_slide -->

Ownership
---
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```rust {all|2|4|6|all} +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // transfer ownership
    let b = v;

    println!("{:?}", b);
}
```
<!-- pause  -->
<!-- column: 1 -->
```rust {2|4-5|all} +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // transfer ownership
    let b = v;
    let c = v;

    println!("{:?}", b);
    println!("{:?}", c);
}
```

<!-- end_slide -->

Immutable borrowing
---
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```rust {all|4-5|all} +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // immutable borrow
    let b = &v;
    let c = &v;

    println!("{:?}", b);
    println!("{:?}", c);
}
```
<!-- pause  -->
<!-- column: 1 -->
```rust {all|7|all} +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // immutable borrow
    let b = &v;
    let c = &v;

    let d = v;
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}
```

<!-- end_slide -->

Mutable borrowing
---
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

```rust {2-6|all} +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;

    b.push(4);

    println!("{:?}", b);
    println!("{:?}", v);
}
```
<!-- pause  -->
<!-- column: 1 -->
```rust {4-8|all} +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    let c = &mut v;

    b.push(4);
    c.push(5);

    println!("{:?}", b);
    println!("{:?}", c);
}
```

<!-- end_slide -->
More mutable borrowing
---
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```rust {4-7|all} +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    let c = &v;

    b.push(4);

    println!("{:?}", b);
    println!("{:?}", c);
}
```
<!-- pause  -->
<!-- column: 1 -->
```rust {4-10|all} +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    b.push(4);
    println!("{:?}", v);

    let c = &mut v;
    c.push(5);
    println!("{:?}", c);

    println!("{:?}", v);
}
```

<!-- end_slide -->
Safe code
---

These rules, enforced at compile time, eliminate entire classes of memory bugs from being possible.

Example of bugs not possible:
- Use after free
- Double free
- Dangling pointers
- Data races (with and without multi-threading)

<!-- end_slide -->

Rust enums
---
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

Rust enums can look like normal enums.
```rust +line_numbers
enum ApplicationState {
    NotStarted,
    Running,
    Finished,
    Failed,
}
```

<!-- pause  -->
<!-- column: 1 -->

But they have a powerful feature: a variant can hold a value.

```rust {all|3,5|all} +line_numbers
enum ApplicationState {
    NotStarted,
    Running(RunningState),
    Finished,
    Failed(String),
}
```
<!-- pause  -->
<!-- column_layout: [1] -->
<!-- column: 0 -->

Rust does not have any `null`-like idiom by itself, but one can use an enum to achieve the same.

<!-- pause  -->
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

This is so common that the standard library includes it:

```rust +line_numbers
enum Option<T> {
    Some(T),
    None,
}
```
<!-- pause  -->
<!-- column: 1 -->
and its sibling
```rust +line_numbers
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
<!-- end_slide -->

Traits
---

A trait describes some functionality or capability. Think of them a bit like Java interfaces.
They are used mostly with generics and are crucial to safety.

<!-- pause  -->

Common traits
- `Copy`
- `Clone`
<!-- pause  -->
- `Iterator`
<!-- pause  -->
- `Sync`
- `Send`
<!-- pause  -->
- `Drop`
<!-- pause  -->
- `From` and `Into`
<!-- pause  -->
- `Default`
<!-- pause  -->
- `Display`
- `Debug`
<!-- pause  -->
- `Hash`
- `Eq` and `PartialEq`
- `Ord` and `PartialOrd`

<!-- end_slide -->
Expressions
---

Rust is an expression-based language. `;` is used to separate expressions. The last expression in a block is returned.
<!-- pause  -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

```rust {all|3-7|all} +exec +line_numbers
fn main() {
    let something = true;
    let a = if something {
        "Good"
    } else {
        "Bad"
    };
    println!("{}", a);
}
```
<!-- pause  -->
<!-- column: 1 -->
```rust +exec +line_numbers
fn main() {
    let something = true;
    let a = match something {
        true => "Good",
        false => "Bad",
    };
    println!("{}", a);
}
```

<!-- end_slide -->
Using enums + generics + statements
---
<!-- pause  -->
<!-- column_layout: [3, 4] -->
<!-- column: 0 -->

```rust {all|2-5|all} +exec +line_numbers
fn check_last(s: &[u32]) {
    match s.last() {  // last() returns Option<&u32>
        Some(entry) => println!("Last slice: {:?}, is: {}.", s, entry),
        None => println!("The slice is empty."),
    };
}

fn main() {
    let v = vec![1, 2, 3];
    check_last(&v);
    check_last(&v[..2]);
    check_last(&[]);
}
```

<!-- column: 1 -->

<!-- pause  -->
There are even specialized versions:
```rust
// in the std::io module
type Result<T> = Result<T, io::Error>;
```

example:
```rust {all|3-8|3|4-7|11-12|all} +exec +line_numbers
use std::{io, fs::File, path::Path};

fn read_length_of_file<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let len = File::open(path)?
        .metadata()?
        .len();
    Ok(len)
}

fn main() -> io::Result<()> {
    println!("File length is: {}", read_length_of_file("presentation.md")?);
    println!("File length is: {}", read_length_of_file("This file does not exist")?);
    Ok(())
}
```

<!-- reset_layout -->
<!-- pause  -->

lambdas

```rust {3-12|all} +exec +line_numbers
use std::{io::{self, BufReader, BufRead}, fs::File, path::Path};

fn number_of_lines_starting_with_f<P: AsRef<Path>>(path: P) -> io::Result<usize> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let count = reader
        .lines()
        .map(Result::unwrap)
        .filter(|line| line.trim().starts_with('f'))
        .count();
    Ok(count)
}

fn main() -> io::Result<()> {
    println!("Lines that start with 'f': {}", number_of_lines_starting_with_f("presentation.md")?);
    Ok(())
}
```

<!-- end_slide -->
Generics
---

```rust +exec +line_numbers
use std::fmt::{Display, Debug};

fn check_last<T: Display + Debug>(s: &[T]) {
    match s.last() {
        Some(entry) => println!("Last of slice: {:?}, is: {}.", s, entry),
        None => println!("The slice is empty."),
    };
}

fn main() {
    let v = vec![1, 2, 3];
    check_last(&v);
    check_last(&v[..2]);
    check_last(&["hello", "world"]);
    check_last::<u8>(&[]);
}
```
<!-- end_slide -->
Strings
---

Rust has many types of strings
- `&str` and `&'static str`
- `String`
- `OsStr`
- `OsString`
- `CStr`
- `CString`
- `Path`
- `PathBuf`

- `&[u8]`
<!-- pause  -->

Rust strings tends to be abit tedoius to work with, but you end up doing it correctly.

<!-- end_slide -->
Cargo
---

Cargo is Rust’s build system and dependency manager.

`cargo new demo_for_presentation`

<!-- pause  -->
The default directory structure:
```bash +exec_replace
cargo clean --manifest-path demo_for_presentation/Cargo.toml > /dev/null 2>&1
rm -f demo_for_presentation/Cargo.lock
eza -Ta --color=always -I demo_for_presentation/target demo_for_presentation
```

<!-- pause  -->
<!-- column_layout: [1, 1] -->
<!-- column: 0 -->

```file +line_numbers
path: demo_for_presentation/src/main.rs
language: rust
```

<!-- pause  -->
<!-- column: 1 -->

```file +line_numbers
path: demo_for_presentation/Cargo.toml
language: toml
```

<!-- reset_layout -->
<!-- pause  -->

running `cargo run --release`

```bash +exec_replace
cargo clean --manifest-path demo_for_presentation/Cargo.toml > /dev/null 2>&1
cargo run --release --manifest-path demo_for_presentation/Cargo.toml --color=always
```

<!-- pause  -->
To install `rustc` and `cargo` use `rustup`. `rustup` is the toolchain manager for Rust.

Get `rustup` from either:
 - [](https://rustup.rs/)
 - Package manager of choice if available

<!-- end_slide -->
Stability
---

Rust guarantees that any code that compiles with a stable version of the compiler will compile with all later versions of the compiler.

<!-- pause  -->

There is a new release of Rust every 6 weeks.

<!-- pause  -->

To ensure that changes can be made, there are editions. The `rustc` compiler knows how to compile its current edition and all older editions.

<!-- pause  -->

The current edition is 2021. The 2024 edition is in its final stages and is going to be released 20 February 2025 in version 1.85.0.

<!-- pause  -->

An edition is defined on a crate level, with cross-edition builds being fully supported.

<!-- pause  -->

Upgrading to the next edition is automated by running `cargo fix --edition`.

<!-- end_slide -->
Why Rust
---

This is what [](https://rust-lang.org) has to say.

#### Performance
- No runtime or garbage collector.
- Easily integrates with other languages.
- Can target everything from preformace heavy / critical devices, to operating systems and down to embedded devices.

<!-- pause  -->
#### Reliability
- Memory-safety.
- Thread-safety.
- Eliminates many classes of bugs at compile-time.

<!-- pause  -->
#### Productivity
- Excellent documentation.
- Friendly compiler & clear error messages.
- Top-notch tooling:
  - Integrated package manager and build tool.
  - Excellent multi-editor support.
  - Auto-completion, type inspections, and auto-formatting.
  - And more...

<!-- end_slide -->
Should you write it in Rust?
---

#### Common blockers
- Missing core library for the target domain.
  - GUIs? [https://areweguiyet.com/](https://areweguiyet.com/)
- Demand for knowledge (developers) currently higher than the supply.

<!-- pause  -->
#### However
- If your target domain includes something that somewhat resembles Rust’s ownership model, you should probably write it in Rust.

<!-- pause  -->
##### Note.
What is not on this slide?

<!-- end_slide -->
`unsafe`
---

A common misconception is that unsafe is an 'escape hatch'.

<!-- pause  -->
Unsafe allows three things.
 1. Use FFI.
 2. Call unsafe functions.
 3. Dereference a raw pointer.

<!-- pause  -->
### The overarching use case
There is some invariant (usually runtime reliant) that makes this idiom safe. (think vector, mutex...)

<!-- pause  -->
### When writing unsafe code
<!-- pause  -->
- One has to make sure that any unsafe code is valid for ANY and ALL safe code.
<!-- pause  -->
- One can use the guarantees / rules of safe Rust code to make safe `unsafe` abstractions.

<!-- end_slide -->
Following the rules of Rust.
---

Because of the rules (especially ownership / borrowing) of Rust, one can get nice APIs that are not possible in other languages.

<!-- pause  -->
#### [`HashMap::entry`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry)
- This API works because the `Entry` type returned by the `entry` function holds a mutable reference to the hashmap.

<!-- pause  -->
#### [`Mutex`](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html)
- The mutex type in Rust takes ownership of the value that it protects.
  - Unlike most other languages where a mutex is just a lock that locks itself, meaning there is nothing connecting the mutex to the value, so it's easy to read the value without having locked the lock.
- The `Mutex::lock` method returns a `MutexGuard` containing a mutable reference to the protected value.
  - When the `MutexGuard` is dropped it releases the lock (similar to C++'s std::lock_guard).
    - As the mutable reference is owned by the `MutexGuard`, there cannot be any references left behind after it's dropped.

<!-- end_slide -->
Why do people like to write Rust?
---

1. Errors at compile time vs. at runtime.
2. Sense that the code they write is correct.
3. Low tradeoffs.

<!-- end_slide -->
Demo time
---

In this demo we are going to do an easy normal task (Fibonacci), see where it fails, then fix it properly.

<!-- end_slide -->
Fibonacci
---

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file {30-34|2-10,15-16,21-24|11,17,25|36-46|all} +line_numbers +exec
path: fib_demo/src/first.rs
language: rust
```
<!-- pause  -->
<!-- column: 1 -->
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=first first
```
<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file {9-15|18-22|24-39|33|24-39|all} +line_numbers +exec
path: fib_demo/src/second.rs
language: rust
```
<!-- pause  -->
<!-- column: 1 -->
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=second second
```

<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file +line_numbers +exec
path: fib_demo/src/third.rs
language: rust
```
<!-- column: 1 -->

<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file +line_numbers
path: fib_demo/src/fourth.rs
language: rust
```
<!-- pause  -->
<!-- column: 1 -->
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=fourth
```
<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file +line_numbers
path: fib_demo/src/fifth.rs
language: rust
```
<!-- pause  -->
<!-- column: 1 -->
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=fifth
```
<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file +line_numbers +exec
path: fib_demo/src/sixth.rs
language: rust
```
<!-- pause  -->
<!-- column: 1 -->
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=sixth
```
<!-- reset_layout -->
<!-- end_slide -->

<!-- column_layout: [1, 1] -->
<!-- column: 0 -->
```file +line_numbers
path: fib_demo/src/seventh.rs
language: rust
```
<!-- column: 1 -->
### Lets add a dependency
```bash
cargo add num
```
Will add this in the dependencies section of the `Cargo.toml` file
```toml
[dependencies]
num = "0.4.3"
```
<!-- pause  -->
### Test
```bash +exec_replace
cargo test --color=always --manifest-path=fib_demo/Cargo.toml --bin=seventh
```
<!-- pause  -->
### Result
```bash +exec_replace
cargo run --color=always --manifest-path=fib_demo/Cargo.toml --bin=seventh
```

<!-- reset_layout -->
<!-- end_slide -->
Closing remarks
---

### If you want to start with Rust:
- Do not be afraid to use clone / unwrap at the start.
- Rust is going to force you to think differently about problems.
- You are going to have times where you "fight" the compiler.
  - If you feel that the compiler really fights you, you should consider how your model is affecting you.
- Program so that only valid states are representable.

<!-- end_slide -->
Sources
---

Sources of slides can be found at [github.com/meltinglava/rust_presentation](https://github.com/meltinglava/rust_presentation)
