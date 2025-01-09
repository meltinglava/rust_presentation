---
title: What is rust?
subtitle: What is this language that everyone talks about, and a look at some features?
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
  
Once the owner goes out of scope, the value is freed.

<!-- end_slide -->

Ownership
---

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // transfer ownership
    let mut b = v;
    
    b.push(4);
    
    println!("{:?}", b);
}
```

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // transfer ownership
    let b = v;
    
    println!("{:?}", b);
    println!("{:?}", v);
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
    
    println!("{:?}", b);
    println!("{:?}", v);
}
```

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // immutable borrow
    let b = &v;
    
    println!("{:?}", v);
    println!("{:?}", b);
}
```

<!-- end_slide -->

Mutable borrowing
---

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    
    b.push(4)

    println!("{:?}", b);
    println!("{:?}", v);
}
```

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
    // mutable borrow
    let b = &mut v;
    let c = &mut v;

    b.push(4);
    c.push(4);

    println!("{:?}", b);
    println!("{:?}", c);
}
```

```rust +exec +line_numbers
fn main() {
    let v = vec![1, 2, 3];
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
