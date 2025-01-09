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

TODO: Explain why this helps you. Also 

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
