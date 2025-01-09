---
title: What is rust?
sub_title: What is this language that everyone talks about, and a look at some features?
author: Roald Strangstadstuen
theme:
  override:
    code:
      alignment: left
      background: false
---

This presentation
---

* What is rust?
* How to do memory safety without a garbage collector?
* Some cool features
* Rust goals (speed, safety and productivity)
* unsafe
* When to use / not use rust
* Demo time

<!-- end_slide -->

What is rust?
---

* Rust is a general purpous, system level programming language
* Was started by Mozilla, with version 1.0 released in May 2015.

<!-- end_slide -->
Rusts memory safety
---

Ownership and borrowing

* All values has one uniqe owner.
* The owner is resposible for freeing the underlying value once it goes out of
  scope.
* There are three ways of values be trasfered:
  1. Transfer of ownership
  2. Immutable boorow(s)
  3. Mutable borrow

Lifetimes

* All owned values has a lifetime
  * The lifetime of all references cannot outlive the lifetime of the owner

<!-- end_slide -->

Ownership
---

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // transfer ownership
    let b = v;

    println!("{:?}", b);
}
```

```rust +exec +line_numbers
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

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // immutable borrow
    let b = &v;
    let c = &v;

    println!("{:?}", b);
    println!("{:?}", c);
}
```

```rust +exec +line_numbers
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

```rust +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;

    b.push(4);

    println!("{:?}", b);
    println!("{:?}", v);
}
```

```rust +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    let c = &mut v;

    b.push(4);
    c.push(4);

    println!("{:?}", b);
    println!("{:?}", c);
}
```

<!-- end_slide -->
More mutable borrowing
---
```rust +exec +line_numbers
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


```rust +exec +line_numbers
fn main() {
    let mut v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    b.push(4);

    let c = &mut v;
    c.push(5);

    println!("{:?}", c);
    println!("{:?}", v);
}
```

TODO: Explain why this helps you.

<!-- end_slide -->

Rust enums
---

Rust enums can look like normal enums.
```rust +line_numbers
enum ApplicationState {
    NotStarted,
    Running,
    Finished,
    Failed,
}
```

But they have a powerful feature that a variant can hold a value.

```rust +line_numbers
enum ApplicationState {
    NotStarted,
    Running(RunningState),
    Finished,
    Failed(String),
}
```

Rust does not have any `null` like ideom by itself, but one can use an enum to atchive the same. This is so common that the standard library includes one:

```rust +line_numbers
enum Option<T> {
    Some(T),
    None,
}
```
and its sibling
```rust +line_numbers
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
<!-- end_slide -->

Using enums
---

```rust +exec +line_numbers
fn check_last(s: &[u32]) {
    match s.last() {  // last() returns Option<u32>
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


There are even specialized versions:
```rust
// in the std::io module
type Result<T> = Result<T, io::Error>;
```


example:
```rust +exec +line_numbers
use std::{io, fs::File, path::Path};

fn read_length_of_file<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let len = File::open(path)?
        .metadata()?
        .len();
    Ok(len)
}

fn main() -> io::Result<()> {
    println!("File length is: {}", read_length_of_file("This file does not exsist")?);
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
Cargo
---

Cargo is rust's build system and dependencies manager.

```bash
cargo new demo_for_presentation
```

The defualt directory structure:
```bash +exec_replace
cargo clean --manifest-path demo_for_presentation/Cargo.toml > /dev/null 2>&1
rm -f demo_for_presentation/Cargo.lock
eza -T --color=always -I demo_for_presentation/target demo_for_presentation # -I does not work
```


```bash +exec_replace
bat --color=always demo_for_presentation/Cargo.toml
```

```bash +exec_replace
bat --color=always demo_for_presentation/src/main.rs
```

running `cargo run --release`

```bash +exec_replace
cargo clean --manifest-path demo_for_presentation/Cargo.toml > /dev/null 2>&1
cargo run --release --manifest-path demo_for_presentation/Cargo.toml --color=always
```

To install `rust` and `cargo` use `rustup`. `rustup` is the toolchain manager for rust.

Get `rustup` from either:
 - Package manager of choise
 - [](https://rustup.rs/)

<!-- end_slide -->
Stability
---

Rust garanties that any code that compiles with a stable version of the compiler, shall compile with all later versions of the compiler.

This means that you can always upgrade your compiler to get the latest features, bug fixes, better build preformace...

There is a new release of rust every 6 weeks.

To ensure that changes can be made, there is edition. The rustc compiler knows how to compile its current edition and all older edition.

The current edition is 2021. The 2024 edition is in its final stages and are going to be released 20 February 2025 in version 1.85.0.

An edition is defined on a crate level, with cross edition builds being fully supported.

Upgrading to the next edition is automated by running `cargo fix --edition`

<!-- end_slide -->
Why Rust
---

This is what [](https://rust-lang.org) has to say.

#### Performance
- No runtime or garbage collector.
- Easily integrates with other languages.
- Can target everything from preformace heavy / critical devices, to operating systems and down to embedded devices.

#### Reliability

- Memory-safety.
- Thread-safety.
- Eliminates many classes of bugs at compile-time.

#### Productivity
Rust enhances developer experience with:
- Excellent documentation.
- Friendly compiler & clear error messages.
- Top-notch tooling:
  - Integrated package manager and build tool.
  - Exelent multi-editor support.
  - Auto-completion, type inspections, and auto-formatting.
  - And more...

<!-- end_slide -->
Should you write it in rust?
---

#### Common blockers
- Missing core library for the target domain.
  - GUI-s [](https://areweguiyet.com/)
- Demand for knowledge (developers) currently higher than the supply.

#### However
- If your target domain includes something that somewhat resembles the ownership model of rust, you should probably write it in rust.

##### Note.
What is not on this slide?

<!-- end_slide -->
`unsafe`
---

A common misconception is that unsafe is an 'escape hatch'.

Unsafe allows three things.
 1. Use FFI.
 2. Call unsafe functions.
 3. Dereference a raw pointer.

### The overarching usecase.
There is some invariant (usually runtime reliant), that makes this ideom safe. (think vector, mutex)


### When writing unsafe code
- One have to make sure that any unsafe code is valid for ANY and ALL safe code. 
- One can use the garanties / rules of safe rust code to make safe `unsafe` abstractions.

<!-- end_slide -->
Following the rules of rust.
---

Because of the rules (espesially ownership / borrowing) of rust, one can get nice api-s that is not possible in other languages. 

#### [`HashMap::entry`](https://doc.rust-lang.org/stable/std/collections/hash_map/struct.HashMap.html#method.entry)
- This API works because the `Entry` type returned by the `entry` function holds a mutable reference to the hashmap.

#### [`Mutex`](https://doc.rust-lang.org/stable/std/sync/struct.Mutex.html)
- The mutex type in rust takes ownership of the value that it protects. 
  - Unlike most other languages a mutex is just a lock that locks itself.
    - Hence there is nothing connecting the mutex to the value and it is easy to suddenly read the value without having locked the lock.
- The `Mutex::lock` method returns a `MutexGuard` containg a mutable reference to the protected value.
  - When the `MutexGuard` is dropped it releases the lock (simular to cpp mutexguard).
    - But it does not ensure that there is no references left behind.

<!-- end_slide -->
Why does people like to write rust.
---

1. Errors at compile time vs at runtime.
2. Sense that the code they write are correct.
3. Low tradeoffs.


<!-- end_slide -->
Demo time
---

In this demo we are going to do an easy normal task, see where it fails, to then fix it properly.

```rust +line_numbers
// main.rs
fn main() {
    let values = 40;
    let sum: usize = fib::Fib::default()
        .take(40)
        .sum();
    println!("Sum of the frist {} values in fibonacci: {}", values, sum);
}
```

```rust +line_numbers
// main.rs
pub struct Fib {
    current: usize,
    former: usize,
    count: usize,
}

impl Fib {
    fn new(current: usize, former: usize) -> Self {
        
    }
}
```
